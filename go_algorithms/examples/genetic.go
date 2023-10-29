package main

import (
	"fmt"
	"math/rand"
	"sort"
	"time"
)

const (
	targetString = "1101101101"
	populationSize = 10        
	mutationRate = 0.01        
)

func main() {
	rand.Seed(time.Now().UnixNano())

	// Generate an initial population of random binary strings
	population := generateInitialPopulation()

	// Main evolution loop
	generation := 0
	for {
		generation++
		fmt.Printf("Generation %d: %s\n", generation, population[0])

		if population[0] == targetString {
			fmt.Println("Target reached!")
			break
		}

		// Select the top 10% of the population to be parents
		parents := selectParents(population)

		// Create a new population through crossover and mutation
		newPopulation := createNewPopulation(parents)

		// Replace the old population with the new one
		population = newPopulation
	}
}

// Generates an initial population of random binary strings
func generateInitialPopulation() []string {
	population := make([]string, populationSize)
	for i := 0; i < populationSize; i++ {
		population[i] = generateRandomString()
	}
	return population
}

// Generates a random binary string of the same length as the target string
func generateRandomString() string {
	str := make([]byte, len(targetString))
	for i := range str {
		if rand.Float64() < 0.5 {
			str[i] = '0'
		} else {
			str[i] = '1'
		}
	}
	return string(str)
}

// Selects the top 10% of the population as parents based on their fitness
func selectParents(population []string) []string {
	// For simplicity, we'll use a fitness function that counts the number of matching bits
	fitness := make(map[string]int)
	for _, candidate := range population {
		score := 0
		for i := range candidate {
			if candidate[i] == targetString[i] {
				score++
			}
		}
		fitness[candidate] = score
	}

	// Sort the population by fitness
	sortByFitness(population, fitness)

	// Select the top 10% as parents
	parentCount := populationSize / 10
	parents := make([]string, parentCount)
	copy(parents, population[:parentCount])

	return parents
}

// Creates a new population through crossover and mutation
func createNewPopulation(parents []string) []string {
	newPopulation := make([]string, populationSize)

	// Keep the best solution from the previous generation
	newPopulation[0] = parents[0]

	// Generate the rest of the new population through crossover and mutation
	for i := 1; i < populationSize; i++ {
		parent1 := parents[rand.Intn(len(parents))]
		parent2 := parents[rand.Intn(len(parents))]
		child := crossover(parent1, parent2)
		child = mutate(child)
		newPopulation[i] = child
	}

	return newPopulation
}

// Perform single-point crossover on two parent strings
func crossover(parent1, parent2 string) string {
	crossoverPoint := rand.Intn(len(parent1))
	return parent1[:crossoverPoint] + parent2[crossoverPoint:]
}

// Mutate a string by flipping some of its bits with a probability
func mutate(str string) string {
	mutated := []byte(str)
	for i := range mutated {
		if rand.Float64() < mutationRate {
			if mutated[i] == '0' {
				mutated[i] = '1'
			} else {
				mutated[i] = '0'
			}
		}
	}
	return string(mutated)
}

// Sort the population based on fitness
func sortByFitness(population []string, fitness map[string]int) {
	sort.SliceStable(population, func(i, j int) bool {
		return fitness[population[i]] > fitness[population[j]]
	})
}
