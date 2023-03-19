use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


#[derive(Deserialize)]  // for serde auto code generation
struct GcdParameters {
    n: u64,
    m: u64,
}


fn main() {

    let server = HttpServer::new(|| {               // rust closure
        App::new()                                  // create an empty new App
            .route("/", web::get().to(get_index))   // how to route request
            .route("/gcd", web::post().to(post_gcd))
    }); // TODO : just one App object?

    println!("Servingon https://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}


fn get_index() -> HttpResponse {                    // response to GET
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            // raw string
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Compute GDC</buton>
                </form>
            "#,
        )
}


fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD  with zero is boring.");
    }

    else {
        let response = 
            format!(
                "The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
                 form.n, form.m, gcd(form.n, form.m)
            );
        
        HttpResponse::Ok()
            .content_type("text/html")
            .body(response)
    }
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
