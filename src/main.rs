use core::cell::RefCell;
use std::rc::Rc;

use slint::SharedString;

slint::include_modules!();

#[derive(Default)]
struct CalcState {
    prev_value: f32,
    current_value: f32,
    operator: slint::SharedString,
}

fn main() {
    let calc = Calculator::new().unwrap();

    let weak = calc.as_weak();

    let state = Rc::new(RefCell::new(CalcState::default()));

    calc.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();

        match value.as_str() {

            "C" => {
                state.current_value = 0.0;
                app.set_value(state.current_value);
            },

            "CE" => {
                state.prev_value = 0.0;
                state.current_value = 0.0;
                state.operator = SharedString::from("");
                app.set_value(state.current_value);
            },

            "del" => {
                let val : f32 = app.get_value();

                let mut sval = val.to_string();
                let sval_len = sval.len();

                if sval_len > 1 {

                    sval = sval[..sval_len].to_string();
                    let new_val :f32 = sval.parse().unwrap();

                    app.set_value(new_val);
                    state.current_value = new_val;

                } else {

                    app.set_value(0.0);
                    state.current_value = 0.0;
                }
            },

            "+" => {
                state.prev_value = state.current_value;
                state.current_value = 0.0;
                state.operator = SharedString::from(value.as_str());
                app.set_value(state.current_value);
            },

            "-" => {
                state.prev_value = state.current_value;
                state.current_value = 0.0;
                state.operator = SharedString::from(value.as_str());
                app.set_value(state.current_value);
            },

            "*" => {
                state.prev_value = state.current_value;
                state.current_value = 0.0;
                state.operator = SharedString::from(value.as_str());
                app.set_value(state.current_value);
            },

            ":" => {
                state.prev_value = state.current_value;
                state.current_value = 0.0;
                state.operator = SharedString::from(value.as_str());
                app.set_value(state.current_value);
            },

            "^2" => {
                let result = pow(state.current_value, 2.0);
                app.set_value(result);
                state.current_value = result;
            },

            "^x" => {
                state.prev_value = state.current_value;
                state.current_value = 0.0;
                state.operator = SharedString::from(value.as_str());
                app.set_value(state.current_value);
            },

            "=" => {
                match state.operator.as_str() {
                    "+" => {
                        let result = state.prev_value + state.current_value;
                        app.set_value(result);
                        state.current_value = result;
                    },
                    "-" => {
                        let result = state.prev_value - state.current_value;
                        app.set_value(result);
                        state.current_value = result;
                    },
                    "*" => {
                        let result = state.prev_value * state.current_value;
                        app.set_value(result);
                        state.current_value = result;
                    },
                    ":" => {
                        let result = state.prev_value / state.current_value;
                        app.set_value(result);
                        state.current_value = result;
                    },
                    "^x" => {
                        let result = pow(state.prev_value, state.current_value);
                        app.set_value(result);
                        state.current_value = result;
                    },
                    _ => app.set_value(state.current_value),
                }
            },

            _ /* if it's a number */ => {
                let val: f32 = value.parse().unwrap();

                state.current_value *= 10.0;
                state.current_value += val;

                app.set_value(state.current_value);
            }

        }
    });

    calc.run().unwrap();
    
}

fn pow(n: f32, m: f32) -> f32 {

    let mut r = 1.0;
    let p : i32 = m.round() as i32;

    if p == 0 {
        return r;
    } else {
        for i in 1..(p+1) {
            r *= n;
        }

        return r;
    }

}
