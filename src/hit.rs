use crate::material::Material;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::rc::Rc;

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
    // TODO virer le RC ?
    pub material: Rc<dyn Material>,
}

impl Hit {
    pub fn new(
        ray: &Ray,
        factor: f64,
        hit_point: Point3,
        outward_normale: Vec3,
        material: Rc<dyn Material>,
    ) -> Hit {
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
                factor,
                material,
            }
        } else {
            Hit {
                hit_point,
                normale: -outward_normale,
                face: Face::Back,
                factor,
                material,
            }
        }
    }
}

//TODO remplacer par Shape/Geometry
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

// TODO sortir le materiau, et les lier dans un Objet(Shape, Material)
pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
    pub(crate) material: Rc<dyn Material>,
}

impl Sphere {
    //une sphere est définit par son centre et son rayon
    pub fn new<T: Material + 'static>(x: f64, y: f64, z: f64, r: f64, material: T) -> Sphere {
        Sphere {
            centre: Point3(x, y, z),
            radius: r,
            material: Rc::new(material),
        }
    }
}

// un slice d'objet qui peuvent intersecter est lui même intersectable.
// l'objet le plus proche avec lequel le rayon intersecte est retenu
impl Hittable for &[Rc<dyn Hittable>] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest = t_max;
        let mut ret = None;
        for obj in self.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                closest = hit.factor;
                ret = Some(hit);
            }
        }
        ret
    }
}

// Pour savoir si une sphere intersect avec le rayon, on recherche un point t sur le Rayon R = (A, B) qui touche la sphere en P(t)
// avec A l'origine du rayon, B le vecteur directeur du rayon
// le point t doit donc satisfaire l'equation de sphere (x2 + y2 + z2 = r2) (donc P(t) est sur la sphere et C le centre de la sphere)
// r2 == vec(P(t)C).vec(P(t)C), et P(t) = A + B*t <==> vec(A+B*t, C).vec(A+B*t, C) == r2
// vec(A+B*t, C) == vec(A,C) + B*t, soit X=vec(A,C) <==> (X + B*t).(X + B*t) == r2
// X.X + 2*t*X.B + t2 * B.B == r2 => eq second degré ax2 +bx +c == 0 => x=t, a=B.B, b=2*x*B, c=X.X
// discriminant delta=b2-4ac, root1=(-b-sqrt(delta))/2a, root2=(-b+sqrt(delta))/2a
// simplication: si b=2h, delta=4h2-4ac, root1=(-2h-sqrt(4h2-4ac))/2a, root2=(-2h+sqrt(4h2-4ac))/2a
//               root1=(-h-sqrt(h2-ac))/a, root2=(h+sqrt(h2-ac))/a
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let x = Vec3::points(self.centre, ray.origin);
        let a = ray.direction.sqr_len();
        let h = x.scalar_product(ray.direction);
        let h2 = h*h;
        let c = x.sqr_len() - self.radius * self.radius;
        let d = h2 - a * c;
        if d >= 0. {
            //2 racines possibles: (-h - d.sqrt()) / a ou (-h + d.sqrt()) / a
            //on ne veut garder que la plus proche, comprise dans l'interval
            //le plus proche de la cam, c'est celui avec la racine la + petite
            // d étant positif, -h - d.sqrt() < -h + d.sqrt(), donc on teste -h - d.sqrt() en premier
            //normale: va du centre  de la sphere vers le hitpoint
            let root = (-h - d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Hit::new(
                    ray,
                    root,
                    ray.at(root),
                    //division par radius plutot que .unit() -> utilisation d'un bug qui reverse la face du matériau en cas de radius negatif
                    Vec3::points(self.centre, ray.at(root)) / self.radius,
                    self.material.clone(),
                ));
            }
            let root = (-h + d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Hit::new(
                    ray,
                    root,
                    ray.at(root),
                    //division par radius plutot que .unit() -> utilisation d'un bug qui reverse la face du matériau en cas de radius negatif
                    Vec3::points(self.centre, ray.at(root)) / self.radius,
                    self.material.clone(),
                ));
            }
        }
        None
    }
}
