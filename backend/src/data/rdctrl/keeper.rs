use sqlx::PgPool;

use crate::db;

use super::transformer::Updates;

// the keeper is here to store all transformed updates into the database ASAP

pub async fn keep(pool: PgPool, updates: Updates) {
    if let Some(weather) = updates.weather {
        tokio::spawn(db::insert::weather(pool.clone(), weather));
    }

    if let Some(extrapolated_clock) = updates.extrapolated_clock {
        tokio::spawn(db::insert::extrapolated_clock(
            pool.clone(),
            extrapolated_clock,
        ));
    }

    if let Some(lap_count) = updates.lap_count {
        tokio::spawn(db::insert::lap_count(pool.clone(), lap_count));
    }

    for team_radio in updates.team_radios {
        tokio::spawn(db::insert::team_radio(pool.clone(), team_radio));
    }

    if let Some(general_timing) = updates.general_timing {
        tokio::spawn(db::insert::general_timing(pool.clone(), general_timing));
    }

    for driver_timing in updates.driver_timings {
        tokio::spawn(db::insert::driver_timing(pool.clone(), driver_timing));
    }

    for driver_sector in updates.driver_sectors {
        tokio::spawn(db::insert::driver_sector(pool.clone(), driver_sector));
    }

    for driver_speed in updates.driver_speeds {
        tokio::spawn(db::insert::driver_speeds(pool.clone(), driver_speed));
    }

    for driver_sector_segment in updates.driver_sector_segments {
        tokio::spawn(db::insert::driver_sector_segment(
            pool.clone(),
            driver_sector_segment,
        ));
    }

    for driver_stint in updates.driver_stints {
        tokio::spawn(db::insert::driver_stint(pool.clone(), driver_stint));
    }

    for driver in updates.drivers {
        tokio::spawn(db::insert::driver(pool.clone(), driver));
    }

    for driver_stats in updates.driver_stats {
        tokio::spawn(db::insert::driver_stats(pool.clone(), driver_stats));
    }

    for driver_sector_stats in updates.driver_sector_stats {
        tokio::spawn(db::insert::driver_sector_stats(
            pool.clone(),
            driver_sector_stats,
        ));
    }

    for car_data in updates.car_data {
        tokio::spawn(db::insert::driver_car_data(pool.clone(), car_data));
    }

    for position in updates.positions {
        tokio::spawn(db::insert::driver_position(pool.clone(), position));
    }
}
