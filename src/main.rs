slint::include_modules!();


const TAXPER: f64=0.30;
const OWNERPER: f64=0.55;
const PRFITPER: f64=0.05;
const OPECPER: f64=0.10;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;


    let ui_handle = ui.as_weak();

   ui.on_divide_income
      (  move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let tax:f64=num*TAXPER;
            let owner:f64=num*OWNERPER;
            let profit:f64=num*PRFITPER;
            let oper:f64=num*OPECPER;
            let result:String=format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpex: {:.2}",{tax},{owner},{profit},{oper});
            ui.set_results(result.into());
        });
  

    ui.run()
}
