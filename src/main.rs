use salvo::{prelude::*, serve_static::StaticDir};


#[handler]
async fn hola_mundo() -> &'static str {
    "Hola, Mundo!"
}

#[handler]
async fn otra_ruta() -> &'static str {
    "Otra ruta"
}

#[handler]
async fn una_mas() -> &'static str {
    "Una ruta mas!"
}


#[tokio::main]
async fn main() {
 
    //cuando llamen con get, lo va a manejar la funcion que pase por parametro, en este caso hola_mundo
    //let router = Router::new().get(hola_mundo);
    //ya que puedo tener varias rutas en el mismo lado, lo guardo con push tipo un array (?

    
    //let router = Router::new()
    //    .push(Router::new().path("").get(hola_mundo))
    //    .push(Router::new().path("otra_ruta").get(otra_ruta))//y asi voy creando varias rutas, cuando accede a otra_ruta ejecuta la funcion
    //    .push(Router::new().path("una_mas").get(una_mas));
    
    //el <**path> es que todo lo que este en la ruta inicial usalo como url (?
    //de ahi vas a buscar en la carpeta public, si no me piden un archivo especifico mostrame index.html
    
    let router = Router::new()
        .push(Router::new().path("").get(hola_mundo))
        .push(Router::new().path("otra_ruta").get(otra_ruta))
        .push(Router::new().path("una_mas").get(una_mas))
        .push(Router::with_path("<**path>").get(StaticDir::new(["public"]).with_defaults("index.html").with_listing(true)));
    
    


    Server::new(TcpListener::bind("127.0.0.1:4000")).serve(router).await;//el localhost 4000
}


