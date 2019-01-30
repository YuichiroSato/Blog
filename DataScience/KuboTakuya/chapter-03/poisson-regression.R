linkFunction <- function(b1, b2) {
  function(x) {
    exp(b1 + b2 * x)
  }
}

distribution <- function(x, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    dpois(y, lambda = l(x))
  }
}

ps <- 0:10
xl <- "y"
yl <- "probability"
legends <- c("x = 0","x = 5","x = 10", "x = 15", "x = 20")
pchs <- c(0, 5, 10, 15, 20)
for (b1 in seq(-0.1, 0.1, by  =  0.05)) {
  for (b2 in seq(-0.1, 0.1, by = 0.05)) {
    l <- linkFunction(b1, b2)
    title <- paste("lambda = exp(", b1, "+", b2, "x)")
    plot(0, 0, type = "n", xlim = c(0, 10), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
    for (x in seq(0, 20, by = 5)) {
      d <- distribution(x, l)
      lines(ps, d(ps), type = "l")
      points(ps, d(ps), pch = x)
    }
    legend("topright", legend = legends, pch = pchs)
  }
}
