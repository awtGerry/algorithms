/* Genetic algorithm to evolve a binary string towards a target string.
 * This is a simple example to demonstrate the basic principles of genetic algorithms.
 * The target string is "1101101101".
 * The population size is 10.
 * The mutation rate is 0.00001.
 * The fitness function is the number of matching characters.
 * The selection function is the top 10% of the population.
 * The crossover function is a single point crossover.
 * The mutation function is a bit flip.
*/

use rand::Rng;
use std::collections::BinaryHeap;

const TARGET_STRING: &str = "1101101101"; // The target binary string we want to evolve towards
const POPULATION_SIZE: usize = 10; // Size of the population
const MUTATION_RATE: f64 = 0.00001; // Probability of a gene mutation

fn main()
{
    let rng = rand::thread_rng();
    let mut population = generate_initial_population();

    let mut generation = 0;
    let mut best_fitness = 0;
    let mut best_individual = String::new();

    loop
    {
        generation += 1;

        // Calculate the fitness of each individual
        let mut fitness_values: Vec<(usize, String)> = Vec::with_capacity(POPULATION_SIZE);
        for candidate in &population
        {
            let fitness = candidate
                .chars()
                .zip(TARGET_STRING.chars())
                .filter(|&(a, b)| a == b)
                .count();
            fitness_values.push((fitness, candidate.clone()));
        }

        // Sort by fitness
        fitness_values.sort_by(|a, b| b.0.cmp(&a.0));

        let (best_fitness_current, best_individual_current) = fitness_values[0].clone();

        println!("Generation {}: {}", generation, best_individual_current);

        if best_fitness_current >= best_fitness
        {
            best_fitness = best_fitness_current;
            best_individual = best_individual_current.clone();
        }

        if best_fitness >= TARGET_STRING.len()
        {
            println!("Target reached!");
            break;
        }

        let parents = select_parents(&fitness_values);

        let new_population = create_new_population(&parents);

        population = new_population;
    }
}

fn generate_initial_population() -> Vec<String>
{
    (0..POPULATION_SIZE)
        .map(|_| generate_random_string())
        .collect()
}

fn generate_random_string() -> String
{
    (0..TARGET_STRING.len())
        .map(|_| {
            if rand::random::<f64>() < 0.5 {
                '0'
            } else {
                '1'
            }
        })
        .collect()
}

fn select_parents(fitness_values: &Vec<(usize, String)>) -> Vec<String>
{
    let mut parents = Vec::new();

    for i in 0..fitness_values.len()
    {
        if i < POPULATION_SIZE / 10
        {
            parents.push(fitness_values[i].1.clone());
        }
        else
        {
            break;
        }
    }

    parents
}

fn create_new_population(parents: &Vec<String>) -> Vec<String>
{
    let mut new_population = Vec::with_capacity(POPULATION_SIZE);
    let mut rng = rand::thread_rng();

    new_population.push(parents[0].clone()); // Keep the best solution

    for _ in 1..POPULATION_SIZE
    {
        let parent1 = &parents[rng.gen_range(0..parents.len())];
        let parent2 = &parents[rng.gen_range(0..parents.len())];
        let child = crossover(parent1, parent2);
        let child = mutate(&child);
        new_population.push(child);
    }

    new_population
}

fn crossover(parent1: &str, parent2: &str) -> String
{
    let mut rng = rand::thread_rng();
    let crossover_point = rng.gen_range(0..parent1.len());
    let mut child = parent1.chars().take(crossover_point).collect::<String>();
    child.push_str(&parent2[crossover_point..]);
    child
}

fn mutate(child: &str) -> String
{
    child
        .chars()
        .map(|c| {
            if rand::random::<f64>() < MUTATION_RATE
            {
                if c == '0' {
                    '1'
                } else {
                    '0'
                }
            }
            else
            {
                c
            }
        })
        .collect()
}
