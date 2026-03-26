/*
By: <Asen Doiron>
Date: 2026-03-09
Program Details: <The purpose of this program is to allow the employee to view what change they should give the customer based on how much cash they give and what item they want to buy.>
*/

mod modules;

use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;
fn window_conf() -> Conf {
    Conf {
        window_title: "storeprogram".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut txt_input1 = TextInput::new(275.0, 125.0, 75.0, 35.0, 25.0);
    let mut txt_input2 = TextInput::new(275.0, 175.0, 75.0, 35.0, 25.0);
    let mut txt_input3 = TextInput::new(275.0, 225.0, 75.0, 35.0, 25.0);
    let mut txt_input4 = TextInput::new(275.0, 275.0, 75.0, 35.0, 25.0);
    let mut txt_input5 = TextInput::new(275.0, 325.0, 75.0, 35.0, 25.0);
    let mut txt_cash = TextInput::new(600.0, 175.0, 150.0, 35.0, 25.0);

    let mut lbl_owed = Label::new("Customer Owes (With Tax):", 75.0, 500.0, 30);

    let mut lbl_help = Label::new("Customer Cash:", 400.0, 200.0, 30);

    let mut lbl_change = Label::new("Customer Change:", 500.0, 250.0, 30);

    let mut lbl_store = Label::new("Bob's General Store!", 75.0, 75.0, 30);

    let mut lbl_apple = Label::new("Apple $3.99", 100.0, 150.0, 30);

    let apple: f64 = 2.99;

    let mut lbl_orange = Label::new("Orange $2.99", 100.0, 200.0, 30);

    let orange: f64 = 2.99;

    let mut lbl_banana = Label::new("Banana $3.50", 100.0, 250.0, 30);

    let banana: f64 = 3.50;

    let mut lbl_watermelon = Label::new("Watermelon $7.99", 100.0, 300.0, 22);

    let watermelon: f64 = 7.99;

    let mut lbl_junoball = Label::new("Juno Ball $19.99", 100.0, 350.0, 22);

    let junoball: f64 = 19.99;

    let btn_exit = TextButton::new(825.0, 25.0, 50.0, 50.0, "X", RED, ORANGE, 30);

    let btn_owe = TextButton::new(135.0, 400.0, 125.0, 50.0, "Check Owed", BLUE, PURPLE, 27);

    let btn_change = TextButton::new(600.0, 100.0, 150.0, 50.0, "Check Change", BLUE, PURPLE, 27);

    let mut owed: f64 = 0.0;

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BROWN);

        if btn_exit.click() {
            break;
        }

        if btn_owe.click() {
            if txt_input1.get_text().is_empty() {
                txt_input1.set_text("0");
            }

            if txt_input2.get_text().is_empty() {
                txt_input2.set_text("0");
            }

            if txt_input3.get_text().is_empty() {
                txt_input3.set_text("0");
            }

            if txt_input4.get_text().is_empty() {
                txt_input4.set_text("0");
            }

            if txt_input5.get_text().is_empty() {
                txt_input5.set_text("0");
            }
            let input1 = txt_input1.get_text().parse::<i32>();
            let input2 = txt_input2.get_text().parse::<i32>();
            let input3 = txt_input3.get_text().parse::<i32>();
            let input4 = txt_input4.get_text().parse::<i32>();
            let input5 = txt_input5.get_text().parse::<i32>();
            if let (Ok(input1), Ok(input2), Ok(input3), Ok(input4), Ok(input5)) = (input1, input2, input3, input4, input5) {
               owed = (apple * input1 as f64) 
                        + (orange * input2 as f64)
                        + (banana * input3 as f64)
                        + (watermelon * input4 as f64)
                        + (junoball * input5 as f64)
                        * 1.13;
                        

                lbl_owed.set_text(format!(
                    "Customer Owes (With Tax): ${:.2}", owed));
            } else {
                lbl_change.set_text("Invalid Input");
            }
        }

        if btn_change.click() {
            if txt_cash.get_text().is_empty() {
                txt_cash.set_text("0");
            }

            let cash: f32 = txt_cash.get_text().trim().parse().unwrap_or(0.0);
            if cash >= owed as f32 {
                lbl_change.set_text(&format!("You can afford this! Your cash back is : ${:.2}", cash - owed as f32));
            } else {
                lbl_change.set_text(&format!(
                    "You cannot afford this! \n All the items Total to:${:.2} you have : ${:.2}",
                    owed, cash
                ));
            }
        }

        txt_input1.draw();
        txt_input2.draw();
        txt_input3.draw();
        txt_input4.draw();
        txt_input5.draw();
        txt_cash.draw();
        lbl_owed.draw();
        lbl_help.draw();
        lbl_change.draw();
        lbl_store.draw();
        lbl_apple.draw();
        lbl_orange.draw();
        lbl_banana.draw();
        lbl_watermelon.draw();
        lbl_junoball.draw();
        next_frame().await;
    }
}
