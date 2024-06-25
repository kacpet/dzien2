use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>>= RefCell::default();
}


#[ic_cdk::query]
fn greet(name: String, last_name: String, wiek : i8) -> String {
    format!("Hello, {} {} {} !", name,last_name,wiek)
}
#[ic_cdk::update]
fn dodaj_wpis(wpis: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow_mut().push(wpis)
    });
}
#[ic_cdk::query]
fn oczytaj_wpisy() -> Vec<String>{
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow().clone()
    })

}
