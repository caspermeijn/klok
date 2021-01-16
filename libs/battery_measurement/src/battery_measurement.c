/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <assert.h>
#include <string.h>

#include "os/os.h"
#include "os/os_callout.h"
#include "os/os_dev.h"
#include "sgm4056/sgm4056.h"
#include "battery/battery.h"
#include "temp/temp.h"
#include "metrics/metrics.h"
#include "log/log.h"

#define LOG_TO_FLASH 1

static temperature_t temperature = 0;
static int32_t battery_voltage_mv = 0;
static charge_control_status_t charger_status = CHARGE_CONTROL_STATUS_OTHER;

/* Define symbols for metrics */
enum {
    BATTERY_METRIC_VAL_CHARGER_STATE,
    BATTERY_METRIC_VAL_VOLTAGE_MV,
    BATTERY_METRIC_VAL_TEMPERATURE_CD, //centi-degrees Celcius
};

/* Define all metrics */
METRICS_SECT_START(battery_metrics)
METRICS_SECT_ENTRY(c, METRICS_TYPE_SINGLE_U)
METRICS_SECT_ENTRY(v, METRICS_TYPE_SINGLE_U)
METRICS_SECT_ENTRY(t, METRICS_TYPE_SINGLE_S)
METRICS_SECT_END;

/* Declare event struct to accommodate all metrics */
METRICS_EVENT_DECLARE(battery_event, battery_metrics);

/* Sample event */
struct battery_event g_event;

static struct log g_log;
#if LOG_TO_FLASH
static struct fcb_log g_log_fcb;
static struct flash_area g_log_fcb_fa;
#else
#define MAX_CBMEM_BUF 10000
static uint8_t cbmem_buf[MAX_CBMEM_BUF];
static struct cbmem cbmem;
#endif

void metrics_init() {
#if LOG_TO_FLASH
    const struct flash_area *fa;
    int rc;

    rc = flash_area_open(FLASH_AREA_NFFS, &fa);
    assert(rc == 0);

    g_log_fcb_fa = *fa;
    g_log_fcb.fl_fcb.f_sectors = &g_log_fcb_fa;
    g_log_fcb.fl_fcb.f_sector_cnt = 1;
    g_log_fcb.fl_fcb.f_magic = 0xBABABABA;
    g_log_fcb.fl_fcb.f_version = g_log_info.li_version;

    g_log_fcb.fl_entries = 0;

    rc = fcb_init(&g_log_fcb.fl_fcb);
    if (rc) {
        flash_area_erase(fa, 0, fa->fa_size);
        rc = fcb_init(&g_log_fcb.fl_fcb);
        assert(rc == 0);
    }

    log_register("battery", &g_log, &log_fcb_handler, &g_log_fcb,
                 LOG_SYSLEVEL);
#else
    cbmem_init(&cbmem, cbmem_buf, MAX_CBMEM_BUF);
    log_register("battery", &g_log, &log_cbmem_handler, &cbmem, LOG_SYSLEVEL);
#endif

    metrics_event_init(&g_event.hdr, battery_metrics, METRICS_SECT_COUNT(battery_metrics), "batt");
    metrics_event_register(&g_event.hdr);
    metrics_set_state_mask(&g_event.hdr, 0xffffffff);
    metrics_event_set_log(&g_event.hdr, &g_log, LOG_MODULE_DEFAULT, LOG_LEVEL_INFO);
}

void metrics_log() {
    uint32_t uptime = os_get_uptime_usec() / 1000 / 1000;
    metrics_event_start(&g_event.hdr, uptime);

    metrics_set_value(&g_event.hdr, BATTERY_METRIC_VAL_CHARGER_STATE, charger_status);
    metrics_set_value(&g_event.hdr, BATTERY_METRIC_VAL_VOLTAGE_MV, battery_voltage_mv);
    metrics_set_value(&g_event.hdr, BATTERY_METRIC_VAL_TEMPERATURE_CD, temperature);

    metrics_event_end(&g_event.hdr);
}

static int
pinetime_battery_prop_changed(struct battery_prop_listener *listener,
                              const struct battery_property *prop)
{
    if(prop->bp_type == BATTERY_PROP_VOLTAGE_NOW) {
        battery_voltage_mv = prop->bp_value.bpv_voltage;
    } else if(prop->bp_type == BATTERY_PROP_SOC) {
        // NOP
    } else {
        assert(false);
    }
    return 0;
}

static struct battery_prop_listener battery_listener = {
        .bpl_prop_read = NULL,
        .bpl_prop_changed = pinetime_battery_prop_changed,
};

static void
pinetime_battery_init(void)
{
    int rc;
    struct os_dev *battery;

    battery = os_dev_open("battery", OS_TIMEOUT_NEVER, NULL);
    assert(battery);

    struct battery_property * prop_voltage = battery_find_property(
            battery, BATTERY_PROP_VOLTAGE_NOW, BATTERY_PROPERTY_FLAGS_NONE, NULL);
    assert(prop_voltage);

    rc = battery_prop_change_subscribe(&battery_listener, prop_voltage);
    assert(rc == 0);

    struct battery_property * prop_soc = battery_find_property(
            battery, BATTERY_PROP_SOC, BATTERY_PROPERTY_FLAGS_NONE, NULL);

    if(prop_soc) {
        rc = battery_prop_change_subscribe(&battery_listener, prop_soc);
        assert(rc == 0);
    }

    rc = battery_set_poll_rate_ms(battery, 30 * 1000);
    assert(rc == 0);
}

static int
charger_data_callback(struct charge_control *chg_ctrl, void * arg,
                      void *data, charge_control_type_t type)
{
    if (type == CHARGE_CONTROL_TYPE_STATUS) {
        charger_status = *(charge_control_status_t*)(data);
    } else {
        assert(false);
    }
    return 0;
}

static struct charge_control_listener charger_listener = {
        .ccl_type = CHARGE_CONTROL_TYPE_STATUS,
        .ccl_func = charger_data_callback,
};

static void
charger_init(void)
{
    int rc;
    struct charge_control *charger;

    charger = charge_control_mgr_find_next_bytype(CHARGE_CONTROL_TYPE_STATUS, NULL);
    assert(charger);

    rc = charge_control_set_poll_rate_ms("charger", 30 * 1000);
    assert(rc == 0);

    rc = charge_control_register_listener(charger, &charger_listener);
    assert(rc == 0);

    rc = charge_control_read(charger, CHARGE_CONTROL_TYPE_STATUS, NULL, NULL, OS_TIMEOUT_NEVER);
    assert(rc == 0);
}

static void
temperature_callback(struct temperature_dev *temp_dev, void * arg)
{
    temperature = temp_get_last_sample(temp_dev);
}

struct temperature_dev *temperature_dev;
static struct os_callout temperature_callout;

static void temperature_periodic(struct os_event *ev);

static void
temperature_init(void)
{
    int rc;

    temperature_dev = (struct temperature_dev *)os_dev_open("temp", OS_TIMEOUT_NEVER, NULL);
    assert(temperature_dev);

    rc = temp_set_callback(temperature_dev, &temperature_callback, NULL);
    assert(rc == 0);

    os_callout_init(&temperature_callout, os_eventq_dflt_get(),
                    temperature_periodic, NULL);

    rc = os_callout_reset(&temperature_callout, 100);
    assert(rc == 0);
}

static void
temperature_periodic(struct os_event *ev)
{
    int rc;

    rc = temp_sample(temperature_dev);
    assert(rc == 0);

    os_time_t ticks = os_time_ms_to_ticks32(30 * 1000);
    rc = os_callout_reset(&temperature_callout, ticks);
    assert(rc == 0);
}

static void periodic_callback(struct os_event *ev);
static struct os_callout periodic_callout;

static void periodic_init()
{
    int rc;

    os_callout_init(&periodic_callout, os_eventq_dflt_get(),
                    periodic_callback, NULL);

    rc = os_callout_reset(&periodic_callout, 1000);
    assert(rc == 0);
}

static void periodic_callback(struct os_event *ev)
{
    int rc;

    metrics_log();

    os_time_t ticks = os_time_ms_to_ticks32(5 * 60 * 1000);
    rc = os_callout_reset(&periodic_callout, ticks);
    assert(rc == 0);
}

void battery_measurement_init()
{
    metrics_init();
    periodic_init();
    charger_init();
    temperature_init();
    pinetime_battery_init();
}
