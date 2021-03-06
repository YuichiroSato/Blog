linkFunction <- function(b1, b2) {
  function(x) {
    exp(b1 + b2 * x)
  }
}

createData <- function(n, b1, b2) {
  x <- runif(n, 1, 20)
  y <- rpois(n, lambda = linkFunction(b1, b2)(x))
  data.frame(x, y)
}

plotGraph <- function(data, fit1, fit2, title) {
  plot(data$x, data$y, type = "p", main = title, xlab = "x", ylab = "y")
  lines(0:20, linkFunction(fit1$coefficient[1], 0)(0:20), lty = "solid")
  lines(0:20, linkFunction(fit2$coefficient[1], fit2$coefficient[2])(0:20), lty = "dashed")
  nl <- paste0("y = exp(", round(fit1$coefficient[1], 3), ")")
  xl <- paste0("y = exp(", round(fit2$coefficient[1], 3), " + ", round(fit2$coefficient[2], 3), " * x)")
  legend("topleft", legend = c(nl, xl), lty = c("solid", "dashed"))
}

plotBootstrap <- function(b1, b2) {
  simulatedData <- createData(100, b1, b2)
  fit1 <- glm(formula = y ~ 1, family = poisson, data = simulatedData)
  fit2 <- glm(formula = y ~ x, family = poisson, data = simulatedData)
  title <- paste0("bootstrap (b1 = ", b1, ", b2 = ", b2, ")")
  plotGraph(simulatedData, fit1, fit2, title)
}

expData <- createData(20, 0.1, 0.07)
fit1 <- glm(formula = y ~ 1, family = poisson, data = expData)
fit2 <- glm(formula = y ~ x, family = poisson, data = expData)
plotGraph(expData, fit1, fit2, "experimental data")

plotBootstrap(fit1$coefficient[1], 0)
plotBootstrap(fit2$coefficient[1], fit2$coefficient[2])
