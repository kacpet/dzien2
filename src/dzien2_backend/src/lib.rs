use std::cell::RefCell;

thread_local! {
    static WPISY: RefCell<Vec<String>> = RefCell::default();
}


#[ic_cdk::query]
fn greet(name: String, last_name: String, iq : i8) -> String {
    format!("Hello, {} {} {} !", name,last_name,iq)
}
#[ic_cdk::update]
fn dodaj_wpis(wpis: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow_mut().push(wpis)
    });
}
#[ic_cdk::query]
fn odczytaj_wpisy() -> Vec<String>{
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow().clone()
    })
}

#[ic_cdk::update]
fn usun_wpis(nr: usize){
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
        wpisy.borrow_mut().remove(nr)
    });
}

#[ic_cdk::update]
fn edytuj_wpis(nr: usize,wpis_nowy: String){
    WPISY.with(|wpisy: &RefCell<Vec<String>>|{
       let mut binding = wpisy.borrow_mut();
       let wpis= binding.get_mut(nr);
       let stary_wpis=  wpis.unwrap();
       *stary_wpis = wpis_nowy;
      

    });
}
