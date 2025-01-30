/*!
  \file selectcontroller.rs
  \brief Contains the SelectController struct for managing fan speed control.

  This module provides a struct for switching between different fan control methods
  based on the specified control method and the current CPU temperature. The
  control methods available are step-based control and soft control.

  \Author:      Logic.Cavalier
  \Date:        2025-01-05
  \Version:     1.0.0
  \License:     AGPLv3+
*/

// Import other modules for step-based control and soft control
use crate::modules::control::confparams::SOFT_CONTROL_METHOD;
use crate::modules::control::confparams::STEP_CONTROL_METHOD;
use crate::modules::control::controlparams::FAN_CONTROL_SPEED_MIN;
use crate::modules::control::softcontroller::SoftController;
use crate::modules::control::stepcontroller::StepController;

/**
* \brief A struct to select and manage fan speed control using either
*        a step-based or soft control method.
*
* This struct provides an abstraction layer to select the control method (step-based
* or soft) for fan speed adjustment. It calculates the fan speed based on the CPU temperature
* and the chosen control method.
*/
pub struct SelectController {
    step_controller: StepController, // Instance of the step-based controller
    soft_controller: SoftController, // Instance of the soft controller
    fan_speed: f64,                  // Stores the calculated fan speed
}

impl SelectController {
    /**
     * \brief Constructor for SelectController.
     *
     * Initializes instances of the step-based and soft-based control structs.
     * The constructor creates objects of StepController and SoftController
     * that will be used to calculate the fan speed based on the control method.
     */
    pub fn new() -> Self {
        SelectController {
            step_controller: StepController::new(),
            soft_controller: SoftController::new(),
            fan_speed: FAN_CONTROL_SPEED_MIN,
        }
    }

    /**
     * \brief Retrieves the current fan speed.
     *
     * This method returns the fan speed that was last calculated based on the
     * selected control method.
     *
     * \returns The fan speed as a f64 value.
     */
    pub fn get_fan_speed(&self) -> f64 {
        self.fan_speed
    }

    /**
     * \brief Selects the appropriate fan control method and calculates fan speed.
     *
     * This method selects which control method to use for calculating the fan speed.
     * It uses the provided control method and the current CPU temperature to
     * determine the fan speed.
     *
     * \param control_method The control method to use. Possible values are "STEP" and "SOFT".
     * \param cpu_temperature The current CPU temperature. This value is used to calculate the fan speed.
     * \returns true if the control method is valid and the fan speed was successfully calculated, false otherwise.
     */
    pub fn select_controller(&mut self, control_method: &str, cpu_temperature: f64) -> bool {
        match control_method {
            STEP_CONTROL_METHOD => {
                self.fan_speed = self.step_controller.calculate_fan_speed(cpu_temperature);
                true
            }
            SOFT_CONTROL_METHOD => {
                self.fan_speed = self.soft_controller.calculate_fan_speed(cpu_temperature);
                true
            }
            _ => {
                self.fan_speed = FAN_CONTROL_SPEED_MIN;
                false
            }
        }
    }
}
