package com.example.demo

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.RestController
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
@RestController
public class DemoApplication {
    @GetMapping("/")
    public fun index(): String {
      return "Hello World!";
    }
}

fun main(args: Array<String>) {
	runApplication<DemoApplication>(*args)
}
