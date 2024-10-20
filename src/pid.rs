use core::time::Duration;

#[cfg(not(feature = "std"))]
use crate::math::Math as _;

/// A Proportional-Integral-Derivative controller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PidController {
    /// The proportional constant.
    pub kp: f64,
    /// The integral constant.
    pub ki: f64,
    /// The derivative constant.
    pub kd: f64,

    /// The range of error values for which the integrator will be taken into account.
    /// If the absolute value of the error is greater than this value, the integrator will be reset to zero and .
    pub integrator_zone: Option<f64>,

    last_position: f64,
    i: f64,
}
impl PidController {
    /// Creates a new PID controller with the given constants.
    ///
    /// # Note
    ///
    /// It is not recommended to use the `ki` constant in a PID controller as it can add instability.
    /// Unless absolutely necessary, use [`Self::new_pd`].
    /// If you do use `ki`, make sure to set `i_zone` to `Some(value)` to prevent integral windup.
    pub fn new_pid(kp: f64, ki: f64, kd: f64, i_zone: Option<f64>) -> Self {
        PidController {
            kp,
            ki,
            kd,
            integrator_zone: i_zone,
            last_position: 0.0,
            i: 0.0,
        }
    }

    /// Creates a new PID controller with the given `kp` and `kd` constants, ommiting `ki`.
    /// This is the recommended way to create a PID controller. However, if you need to use `ki`, use [`Self::new_pid`].
    pub fn new_pd(kp: f64, kd: f64) -> Self {
        PidController {
            kp,
            ki: 0.0,
            kd,
            integrator_zone: None,
            last_position: 0.0,
            i: 0.0,
        }
    }

    /// Updates the PID controller with the given setpoint, current state, and delta time.
    ///
    /// # Panics
    ///
    /// Panics if `dt` is 0.
    pub fn update(&mut self, setpoint: f64, state: f64, dt: Duration) -> f64 {
        assert!(
            !dt.is_zero(),
            "PID update called with a nonsensical delta time of 0"
        );
        let dt = dt.as_secs_f64();

        let error = setpoint - state;

        // If the error is outside of the integrator zone, reset the integrator.
        if self
            .integrator_zone
            .map(|i_zone| error.abs() > i_zone)
            .unwrap_or(false)
        {
            self.i = 0.0;
        } else {
            self.i += error * dt;
        }

        let p = self.kp * error;
        let i = self.ki * self.i;
        let d = self.kd * (state - self.last_position) / dt;
        let output = p + i + d;

        self.last_position = state;

        output
    }
}

#[cfg(test)]
mod tests {
    const SECOND: core::time::Duration = core::time::Duration::from_secs(1);

    #[test]
    fn update() {
        let mut pid = super::PidController::new_pid(1.0, 0.0, 0.0, None);
        assert_eq!(pid.update(0.0, 0.0, SECOND), 0.0);
        assert_eq!(pid.update(0.0, 1.0, SECOND), -1.0);
        assert_eq!(pid.update(0.0, 0.0, SECOND), 0.0);
    }

    #[test]
    #[should_panic]
    fn update_zero_dt() {
        let mut pid = super::PidController::new_pid(1.0, 0.0, 0.0, None);
        pid.update(0.0, 0.0, core::time::Duration::ZERO);
    }

    #[test]
    fn i_zone() {
        let mut pid = super::PidController::new_pid(0.0, 1.0, 0.0, Some(1.0));

        assert_eq!(pid.update(0.0, 3.0, SECOND), 0.0);
        assert_eq!(pid.update(0.0, 0.25, SECOND), -0.25);
        assert_eq!(pid.update(0.0, 0.25, SECOND), -0.5);
        assert_eq!(pid.update(0.0, -3.0, SECOND), 0.0);
    }
}
