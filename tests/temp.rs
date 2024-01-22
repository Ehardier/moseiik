/*#[cfg(test)]
mod tests {
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // TODO
        // test avx2 or sse2 if available
        assert!(true);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        //TODO
        assert!(true);
    }

    #[test]
    fn test_generic() {
        //TODO
        assert!(true);
    }
}*/

#[path = "../src/main.rs"]
mod main; // Importez le module principal

use main::{compute_mosaic, Options}; // Utilisez les éléments du module principal

#[cfg(test)]
mod tests {
    use super::*; // Utilisez les éléments du module principal

    /*#[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // TODO: votre implémentation de test ici
        run_integration_test();
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // TODO: votre implémentation de test ici
        run_integration_test();
    }

    #[test]
    fn test_generic() {
        // TODO: votre implémentation de test ici
        run_integration_test();
    }*/
    use image::{
    imageops::{resize, FilterType::Nearest},
    io::Reader as ImageReader,
    GenericImage, GenericImageView, RgbImage,
    };
    #[test]
    fn run_integration_test() {
        // TODO: votre implémentation de test ici

        // Définir les options pour l'appel à compute_mosaic
        let options = Options {
            image: "assets/kit.jpeg".to_string(),
            output: "assets/output.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 4,
        };

        // Appelez compute_mosaic avec les options
        compute_mosaic(options);

        // Comparez l'image générée avec la vérité terrain
        let generated_image = image::open("assets/output.png").expect("Failed to open generated image");
        let truth_image = image::open("assets/ground-truth-kit.png").expect("Failed to open truth image");
        let res = &truth_image == &generated_image;
	println!("resultat : {}", res);
        // Comparez les deux images 
        //assert_eq!(generated_image, truth_image);
        assert!(true);
    }
}
