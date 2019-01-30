createData <- function() {
  y <- rpois(10, lambda = 4)
  data.frame(y)
}

logLikelihood <- function(data, b1) {
  sum(log(dpois(data, lambda = exp(b1))))
}

printLogLikelihood <- function(data, i) {
  cat(paste0("Input data is data", i, "\n"))
  print(data[i]$y)
  
  fit <- glm(formula = y ~ 1, family = poisson, data = data[i])
  b1 <- fit$coefficients[1]
  
  cat("Estimated b1 =", b1, "\n")
  cat("Log likelihood for\n")
  cat("data1\t\tdata2\t\tdata3\t\n")
  cat(logLikelihood(data[1]$y, b1), "\t", logLikelihood(data[2]$y, b1), "\t", logLikelihood(data[3]$y, b1), "\n\n")
}

printData <- function(data, i) {
  cat(paste0("data", i))
  print(data[i]$y)
}

data <- c(createData(), createData(), createData())

for(i in 1:3) {
  printData(data, i)
}
cat("\n")
for(i in 1:3) {
  printLogLikelihood(data, i)
}
