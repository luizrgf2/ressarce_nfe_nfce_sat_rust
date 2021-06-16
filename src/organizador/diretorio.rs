use std::fs;

pub struct Arquivos{


    pub xmls : Vec<String>,
    pub speds : Vec<String>

}

impl Arquivos {


    fn adc_sped(&mut self,sped : String){

        self.speds.push(sped);

    }
    
    fn adc_xml(&mut self,xml : String){

        self.xmls.push(xml);

    }
}


fn ler_diretorios(path: &str) ->Vec<String>{

    let diretorios = fs::read_dir(path).unwrap();

    let mut arquivos = Vec::new();

    for diretorio in diretorios{


        let dir_atual =diretorio.unwrap().path().clone(); //diretorio atual

        if dir_atual.is_dir(){

            let arquivos_tmp = ler_diretorios(&dir_atual.display().to_string());
            
            for arquivo in arquivos_tmp{

                arquivos.push(arquivo);
            }
        
       
        }else{



                
            arquivos.push(dir_atual.display().to_string());
            


        }


    }

    arquivos


}

pub fn pegar_xmls_speds() ->Arquivos{


    let mut arquivos = Arquivos{

        speds:Vec::new(),
        xmls:Vec::new()

    };

    let diretorios = ler_diretorios("./arquivos");

    for diretorio in diretorios{

        let dir_atual = diretorio;


        if dir_atual.contains(".xml"){

            arquivos.adc_xml(dir_atual);
            
            
        }else if dir_atual.contains(".txt"){
            
            
            arquivos.adc_sped(dir_atual);

        }

    };


    arquivos


}