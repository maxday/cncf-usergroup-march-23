package main

import (
	"fmt"
	"io"
	"math/rand"
	"net/http"

	"github.com/gin-gonic/gin"
	"go.opentelemetry.io/contrib/instrumentation/github.com/gin-gonic/gin/otelgin"
	"go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp"
)

type pizza struct {
	ID       string   `json:"id"`
	Name     string   `json:"name"`
	Toppings []string `json:"toppings"`
	Price    float64  `json:"price"`
}

var pizzas = []pizza{
	{ID: "0", Name: "margherita", Toppings: []string{"tomato sauce, fresh mozzarella and basil"}, Price: 10.50},
	{ID: "1", Name: "deluxe", Toppings: []string{"tomato sauce, bacon, mushrooms"}, Price: 15.50},
	{ID: "2", Name: "veggie", Toppings: []string{"tomato sauce, green pepper, onion, spinach"}, Price: 12.50},
}

func main() {
	shutdown := initProvider()
	defer shutdown()
	router := gin.Default()
	router.Use(otelgin.Middleware("pizza-order"))
	router.GET("/pizzas", getPizzas)
	router.GET("/pizza/:id", getPizzaById)
	router.Run(":7080")
}

func getPizzas(c *gin.Context) {
	if rand.Intn(100) < 20 {
		c.AbortWithStatus(500)
		return
	}
	c.IndentedJSON(http.StatusOK, pizzas)
}

func getPizzaById(c *gin.Context) {
	id := c.Param("id")
	url := fmt.Sprintf("http://pizza-details:7081/pizza/%s", id)
	resp, err := otelhttp.Get(c.Request.Context(), url)
	if err != nil {
		c.AbortWithStatus(http.StatusBadGateway)
		return
	}
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		c.AbortWithStatus(http.StatusInternalServerError)
		return
	}
	c.IndentedJSON(http.StatusOK, gin.H{"details:": string(body)})
}
