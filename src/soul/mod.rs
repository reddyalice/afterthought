use crate::prelude::*;

impl Soul {
    pub fn set_sphere(&mut self, sphere: &Sphere, level: i8) {
        let level = level & 7;
        match sphere {
            Sphere::Force => self.spheres[0] = (self.spheres[0] & -8) + level,
            Sphere::Relation => self.spheres[0] = (self.spheres[0] & -57) + (level << 3),
            Sphere::Matter => {
                self.spheres[0] = (self.spheres[0] & 63) + (level << 6);
                self.spheres[1] = (self.spheres[0] & -2) + (level & 4);
            }
            Sphere::Life => self.spheres[1] = (self.spheres[1] & -15) + (level << 1),
            Sphere::Entropy => self.spheres[1] = (self.spheres[1] & -113) + (level << 4),
            Sphere::Spirit => {
                self.spheres[1] = (self.spheres[1] & 127) + (level << 7);
                self.spheres[2] = (self.spheres[2] & -4) + ((level & 6) >> 1);
            }
            Sphere::Mind => self.spheres[2] = (self.spheres[2] & -29) + (level << 2),
            Sphere::Time => self.spheres[2] = (self.spheres[2] & 31) + (level << 5),
        }
    }

    pub fn get_sphere(&self, sphere: &Sphere) -> i8 {
        match sphere {
            Sphere::Force => self.spheres[0] & 7,
            Sphere::Relation => (self.spheres[0] & 56) >> 3,
            Sphere::Matter => ((self.spheres[1] & 1) << 2) + ((self.spheres[0] & 63) >> 6),
            Sphere::Life => (self.spheres[1] & 14) >> 1,
            Sphere::Entropy => (self.spheres[1] & 112) >> 4,
            Sphere::Spirit => ((self.spheres[2] & 3) << 1) + ((self.spheres[1] & -128) >> 7),
            Sphere::Mind => (self.spheres[2] & 28) >> 2,
            Sphere::Time => (self.spheres[2] & -32) >> 5,
        }
    }

    pub fn add_to_sphere(&mut self, sphere: &Sphere, val: i8) {
        let curr = self.get_sphere(sphere) + val;
        if curr > 7 { self.set_sphere(sphere, 7); }
        else if curr < 0 { self.set_sphere(sphere, 0); }
        else { self.set_sphere(sphere, curr); }
        
    }
}

#[derive(Component)]
pub struct Soul {
    spheres: [i8; 3],
}

pub enum Sphere {
    Force,
    Relation,
    Matter,
    Life,
    Entropy,
    Spirit,
    Mind,
    Time,
}
