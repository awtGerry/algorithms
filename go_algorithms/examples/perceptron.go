package main

import (
	"fmt"
	"math/rand"
	"time"

	"github.com/fogleman/gg"
)

const (
	WIDTH  = 800
	HEIGHT = 800
)

type Point struct {
	X, Y float64
}

type Perceptron struct {
	Weights   [2]float64
	Bias      float64
	LearningRate float64
}

func NewPerceptron(learningRate float64) Perceptron {
	return Perceptron{
		Weights:      [2]float64{rand.Float64(), rand.Float64()},
		Bias:         rand.Float64(),
		LearningRate: learningRate,
	}
}

func (p *Perceptron) Train(inputs []float64, target float64) {
	// Calculate the predicted value
	sum := p.Weights[0]*inputs[0] + p.Weights[1]*inputs[1] + p.Bias
	predicted := sign(sum)

	// Update the weights and bias
	error := target - predicted
	p.Weights[0] += p.LearningRate * error * inputs[0]
	p.Weights[1] += p.LearningRate * error * inputs[1]
	p.Bias += p.LearningRate * error
}

func sign(x float64) float64 {
	if x >= 0 {
		return 1
	}
	return -1
}

func main() {
	rand.Seed(time.Now().UnixNano())

	// Create a Perceptron with a learning rate
	perceptron := NewPerceptron(0.01)

	// Create a window
	dc := gg.NewContext(WIDTH, HEIGHT)
	dc.SetRGB(1, 1, 1)
	dc.Clear()

	// Train the Perceptron to learn the OR gate
	trainingData := [][3]float64{
		{0, 0, -1},
		{0, 1, 1},
		{1, 0, 1},
		{1, 1, 1},
	}

	for i := 0; i < 1000; i++ {
		data := trainingData[rand.Intn(len(trainingData))]
		inputs := data[:2]
		target := data[2]

		perceptron.Train(inputs, target)
	}

	// Visualize the decision boundary
	for x := 0; x < WIDTH; x += 20 {
		for y := 0; y < HEIGHT; y += 20 {
			inputs := []float64{float64(x) / WIDTH, float64(y) / HEIGHT}
			predicted := perceptron.Weights[0]*inputs[0] + perceptron.Weights[1]*inputs[1] + perceptron.Bias

			if sign(predicted) == 1 {
				dc.SetRGB(0, 1, 0)
			} else {
				dc.SetRGB(1, 0, 0)
			}

			dc.DrawRectangle(float64(x), float64(y), 20, 20)
			dc.Fill()
		}
	}

	fmt.Println("Close the window to exit.")
	dc.SavePNG("perceptron.png") // Save the visualization to a file
	dc.Stroke()
	dc.Stroke()
	dc.Stroke()
}
