mod system;
mod consts;
mod dust;
mod enviro;
mod planetismal;
mod utils;

use system::System;
use enviro::*;
use planetismal::Planetismal;
use serde_json::json;

#[derive(Debug)]
pub enum AccreteOutput {
    Struct(Vec<Planetismal>),
    Json(String),
}

pub fn run(
    /// Limit number of planets
    planets_limit: Option<u8>,

    /// Preconfigured stellar mass
    stellar_mass: Option<f64>,

    /// "A" in Dole's paper
    /// Dole's paper tests ranges between 0.00125 and 0.0015
    /// Binary stars produced by increasing coeff of dust density in cloud (Formation of Planetary Systems by Aggregation: A Computer Simulation by Stephen H. Dole)
    /// Range: 0.00125-0.0015
    /// Default: 0.0015
    DUST_DENSITY_COEFF: Option<f64>,
   
    /// The dust-to-gas ratio 50-100 (dust/gas = K), gas = hydrogen and helium, dust = other
    /// Range: 50.0-100.0
    /// Default: 50.0
    K: Option<f64>,

    /// Eccentricity of dust cloud
    /// Range: 0.15-0.25
    /// Default: 0.20
    DUST_CLOUD_ECCENTRICITY: Option<f64>,

    /// Crit_mass coeff
    /// Range: 1.0e-5 - 1.2e-5
    /// Default: 1.2e-5
    B: Option<f64>,
    
    /// Enable moon generation by accretion and collision
    with_moons: bool,

    /// Return json instead of struct
    to_json: bool,
) -> AccreteOutput {
    // var anum;
    // var main_seq_life;
    // var age, r_ecosphere;
    // var r_greenhouse;
    // var spin_resonance;
    Let mut system = System::set_initial_conditions();
    system.distribute_planetary_masses();
    // let main_seq_life = 1.0E10 * (stellar_mass_ratio / stellar_luminosity_ratio);
    // if ((main_seq_life >= 6.0E9))
    // age = random_number(1.0E9, 6.0E9);
    // else
    // age = random_number(1.0E9, main_seq_life);
    // r_ecosphere = Math.sqrt(stellar_luminosity_ratio);
    // r_greenhouse = r_ecosphere * GREENHOUSE_EFFECT_CONST;

    // while (planet != NULL) {
    // planet.orbit_zone = orbital_zone(planet.a);
    // if (planet.gas_giant) {
    //     planet.density = empirical_density(planet.mass, planet.a, planet.gas_giant);
    //     planet.radius = volume_radius(planet.mass, planet.density);
    // } else {
    //     planet.radius = kothari_radius(planet.mass, planet.a, planet.gas_giant, planet.orbit_zone);
    //     planet.density = volume_density(planet.mass, planet.radius);
    // }
    // planet.orbital_period = period(planet.a, planet.mass, stellar_mass_ratio);
    // planet.day = day_length(planet.mass, planet.radius, planet.orbital_period, planet.e, planet.gas_giant);
    // planet.resonant_period = spin_resonance;
    // planet.axial_tilt = inclination(planet.a);
    // planet.escape_velocity = escape_vel(planet.mass, planet.radius);
    // planet.surface_accel = acceleration(planet.mass, planet.radius);
    // planet.rms_velocity = rms_vel(MOLECULAR_NITROGEN, planet.a);
    // planet.molecule_weight = molecule_limit(planet.a, planet.mass, planet.radius);
    // if ((planet.gas_giant)) {
    //     planet.surface_grav = INCREDIBLY_LARGE_NUMBER;
    //     planet.greenhouse_effect = FALSE;
    //     planet.volatile_gas_inventory = INCREDIBLY_LARGE_NUMBER;
    //     planet.surface_pressure = INCREDIBLY_LARGE_NUMBER;
    //     planet.boil_point = INCREDIBLY_LARGE_NUMBER;
    //     planet.hydrosphere = INCREDIBLY_LARGE_NUMBER;
    //     planet.albedo = about(GAS_GIANT_ALBEDO, 0.1);
    //     planet.surface_temp = INCREDIBLY_LARGE_NUMBER;
    // } else {
    //     planet.surface_grav = gravity(planet.surface_accel);
    //     planet.greenhouse_effect = greenhouse(planet.orbit_zone, planet.a, r_greenhouse);
    //     planet.volatile_gas_inventory = vol_inventory(planet.mass, planet.escape_velocity, planet.rms_velocity, stellar_mass_ratio, planet.orbit_zone, planet.greenhouse_effect);
    //     planet.surface_pressure = pressure(planet.volatile_gas_inventory, planet.radius, planet.surface_grav);
    //     if ((planet.surface_pressure == 0.0))
    //     planet.boil_point = 0.0;
    //     else
    //     planet.boil_point = boiling_point(planet.surface_pressure);
    //     iterate_surface_temp(planet);
    // }
    // planet = planet.next_planet;
    // }
    if to_json {
        let s = json!({
            // "stellar_mass": stellar_mass,
            // "stellar_luminosity": stellar_luminosity,
            "planets": system.planets,
        })
        .to_string();
        return AccreteOutput::Json(s);
    }
    println!("{:#?}", system);
    AccreteOutput::Struct(system.planets)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_it() {
        run(false);
    }
}
