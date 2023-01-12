#[allow(dead_code)]
pub struct Materia {
    nombre: String,
    creditos_requeridos: f32,
    factor_creditos: f32 // Servira para multiplicarlo por una nota y brindar los creditos
}

impl Materia {

    pub fn new(nombre: String, creditos_requeridos: f32, factor_creditos: f32) -> Self {
        if creditos_requeridos < 0.0 || creditos_requeridos > 10.0 {
            panic!("Creditos requeridos deben ser mayor a 0.0 y menor o igual que 10.0")
        } else if  factor_creditos < 0.1 || factor_creditos > 1.0 {
            panic!("Factor de creditos debe ser mayor o igual a 0.1 y menor o igual que 1.0")
        } else {
            Self{
                nombre,
                creditos_requeridos,
                factor_creditos
            }
        }
    }

    pub fn puede_inscribirse(&self, creditos_estudiante: f32) -> bool{
        creditos_estudiante >= self.creditos_requeridos
    }
}

pub struct Calificacion {
    materia: Materia,
    nota: f32
}

impl Calificacion {
    pub fn creditos_obtenidos(&self) -> f32 {
        self.materia.factor_creditos * self.nota
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn puede_inscribirse_si_los_creditos_son_mayores_a_los_requeridos() {
        let materia = Materia::new(
            String::from("matematicas"),
            10.0,
            0.5
        );

        let creditos_a_evaluar = 11.0;
        let resultado_obtenido = materia.puede_inscribirse(creditos_a_evaluar);

        assert_eq!(resultado_obtenido, true);
    }

    #[test]
    fn puede_inscribirse_si_los_creditos_son_iguales_a_los_requeridos() {
        let materia = Materia::new(
            String::from("matematicas"),
            10.0,
            0.5
        );

        let creditos_a_evaluar = 10.0;
        let resultado_obtenido = materia.puede_inscribirse(creditos_a_evaluar);

        assert_eq!(resultado_obtenido, true);
    }

    #[test]
    fn no_puede_inscribirse_si_los_creditos_son_iguales_a_los_requeridos() {
        let materia = Materia::new(
            String::from("matematicas"),
            10.0,
            0.5
        );

        let creditos_a_evaluar = 1.0;
        let resultado_obtenido = materia.puede_inscribirse(creditos_a_evaluar);

        assert_eq!(resultado_obtenido, false);
    }

    #[test]
    #[should_panic(expected = "Creditos requeridos deben ser mayor")]
    fn levanta_un_error_si_los_creditos_proporcionados_no_estan_en_el_rango_esperado() {
        let _materia = Materia::new(
            String::from("matematicas"),
            -10.0,
            0.5
        );
    }

    #[test]
    #[should_panic(expected = "Factor de creditos debe ser mayor o igual")]
    fn levanta_un_error_si_factor_de_credito_no_esta_en_el_rango_esperado() {
        let _materia = Materia::new(
            String::from("matematicas"),
            9.0,
            -0.5
        );
    }

}
