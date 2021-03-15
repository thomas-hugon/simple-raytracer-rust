use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;

pub enum Face {
    Front,
    Back,
}
pub struct Hit {
    pub hit_point: Point3,
    //la normale est toujours stocké de sens opposé par rapport au rayon lancé
    pub normale: Vec3,
    pub face: Face,
    pub factor: f64,
}

impl Hit {
    pub fn new(ray: &Ray, factor: f64, hit_point: Point3, outward_normale: Vec3) -> Hit {
        // normale: centre -> hitpoint
        // si rayon sens opposé par rapport à normale -> on voit en direction du centre, donc la face ext
        // sinon rayon meme sens: la cam est entre le centre et le hitpoint donc face int
        // meme sens si produit scalaire > 0
        // si la normale.rayon
        if ray.direction.scalar_product(outward_normale) < 0. {
            Hit {
                hit_point,
                normale: outward_normale,
                face: Face::Front,
                factor
            }
        } else {
            Hit {
                hit_point,
                normale: -outward_normale,
                face: Face::Back,
                factor
            }
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
}

impl Hittable for &[Box<dyn Hittable>]{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest = t_max;
        let mut ret = None;
        for obj in self.iter(){
            if let Some(hit) = obj.hit(ray, t_min, closest){
                closest = hit.factor;
                ret = Some(hit);
            }
        }
        ret
    }
}

impl Hittable for Sphere {
    /*fn hit_sphere(sphere_center: Point3, sphere_radius: f64, ray: &Ray) -> Option<Point3> {
        // P: point(x, y, z), C: origin; P(t) = A + tb => A: origine du rayon, b: direction du rayon, t: un facteur de distance (Ray::at)

        // on recherche un point t sur le Rayon qui touche la sphere
        // => qui satisfait l'equation de la sphere (x2 + y2 + z2 = r2)
        // donc (P(t)-C).(P(t)-C) == r2 <==> (A + tb - C).(A + tb - C) == r2
        // ( (A-C) +tb ).( (A-C) +tb ) == (A-C).(A-C) + tb.(A-C) + tb.(A-C) + tb.tb == r2
        //X = A-C == sphere_center_to_origin
        //
        // X.X + 2*t*b.X + t2*b.b - r2 == 0 => eq du second degré ax2 + bx + c == 0 => a=b.b, b=2*b.X, c=X.X - r2
        // discriminant : delta = b2-4ac.
        // 1 solution si delta == 0, 2 si delta > 0 sinon 0 solution

        let sphere_center_to_origin = Vec3::points(sphere_center, ray.origin);
        let squared_radius = sphere_radius * sphere_radius;

        let a = ray.direction.scalar_product(ray.direction);
        let b = 2.0 * sphere_center_to_origin.scalar_product(ray.direction);
        let c = sphere_center_to_origin.scalar_product(sphere_center_to_origin) - squared_radius;
        let discriminant = b * b - 4f64 * a * c;
        if discriminant < 0. {
            None
        } else {
            Some(ray.at((-b - discriminant.sqrt()) / (2. * a)))
        }
        // discriminant == 0. && b < 0. || discriminant > 0. && b*b > discriminant
    }*/
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let x = Vec3::points(self.centre, ray.origin);
        let a = ray.direction.sqr_len();
        let h = x.scalar_product(ray.direction);
        let c = x.sqr_len() - self.radius * self.radius;
        let d = h * h - a * c;
        if d >= 0. {
            //2 racines possibles: (-h - d.sqrt()) / a ou (-h + d.sqrt()) / a
            //on ne veut garder que la plus proche, comprise dans l'interval
            //le plus proche de la cam, c'est celui avec la racine la + petite
            // d étant positif, -h - d.sqrt() < -h + d.sqrt(), donc on teste -h - d.sqrt() en premier
            //normale: va du centre  de la sphere vers le hitpoint
            let root = (-h - d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Hit::new(ray, root, ray.at(root), Vec3::points(self.centre, ray.at(root)).unit()))
            }
            let root = (-h + d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Hit::new(ray, root, ray.at(root), Vec3::points(self.centre, ray.at(root)).unit()))
            }
        }
        None
    }
}
