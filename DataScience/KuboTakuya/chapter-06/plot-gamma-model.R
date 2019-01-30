linkFunction <- function(b1, b2) {
  function(x) {
    exp(b1 + b2 * log(x))
  }
}

distribution <- function(x, s, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    dgamma(y, shape = s, rate = s / l(x))
  }
}

xs <- c(0.1, 1.0, 2.0, 3.0, 4.0)
ys <- seq(0.5, 10.0, by = 0.5)
pchs <- 1:5
xl <- "y"
yl <- "Probability"
legends <- paste0("x = ", xs)
for (b1 in seq(0.5, 2.5, by = 0.5)) {
  for (b2 in seq(0.5, 2.5, by = 0.5)) {
    for (s in 1:5) {
      l <- linkFunction(b1, b2)
      title <- paste0("shape = ", s, ", rate = ", s, " / exp(", b1, " + ", b2, "log(x))")
      plot(0, 0, type = "n", xlim = c(0, 10), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
      for (i in 1:5) {
        d <- distribution(xs[i], s, l)
        lines(ys, d(ys), type = "l")
        points(ys, d(ys), pch = pchs[i])
      }
      legend("topright", legend = legends, pch = pchs)
    }
  }
}
