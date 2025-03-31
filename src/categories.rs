use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Object: Clone + Eq + Hash + Debug {}

pub trait Morphism {
    type Domain: Object + ?Sized;
    type Codomain: Object + ?Sized;
    fn domain(&self) -> &Self::Domain;
    fn codomain(&self) -> &Self::Codomain;
    fn map(&self, domain: &Self::Domain) -> Self::Codomain;
}

pub fn check_eq_morphisms<A: Object, B: Object>(first: &Box<dyn Morphism<Domain = A, Codomain = B>>, second: &Box<dyn Morphism<Domain = A, Codomain = B>>) -> bool {
    if first.domain() == second.domain() && first.codomain() == second.codomain() && first.map(first.domain()) == second.map(second.domain()) {
        return true
    }
    false
}

pub fn compose<A: Object, B: Object, C: Object>(domain: &A, first: &Box<dyn Morphism<Domain = A, Codomain = B>>, second: &Box<dyn Morphism<Domain = B, Codomain = C>>) -> C {
    second.map(&first.map(domain))
}

type HomSet<A, B> = Vec<Box<dyn Morphism<Domain = A, Codomain = B>>>;

/// A `Category` of a single class of object
/// e.g. Vect_k, Hilb_k
pub struct Category<O: Object + ?Sized> {
    objects: Vec<O>,
    morphisms: HashMap<(usize, usize), HomSet<O, O>>
}

impl<O: Object + ?Sized> Category<O> {
    pub fn create() -> Self {
        Category { objects: Vec::new(), morphisms: HashMap::new() }
    }
    pub fn from_object_list(objects: &[O]) -> Self {
        Category { objects: objects.to_vec(), morphisms: HashMap::new() }
    }

    pub fn add_object(&mut self, object: O) {
        if !self.objects.contains(&object) {
            self.objects.push(object)
        }
    }

    pub fn add_morphism(&mut self, domain: usize, codomain: usize, map: Box<dyn Morphism<Domain = O, Codomain = O>>) {
        let result = self.morphisms.get_mut(&(domain, codomain));
        if result.is_none() {
            self.morphisms.insert((domain, codomain), vec![map]);
            return;
        }
        let mut insert = true;
        let homset = result.unwrap();
        homset.iter().for_each(|x| {
            if check_eq_morphisms(x, &map) {
                insert = false;
            }
        });
        if insert {
            homset.push(map);
        }
    }
}