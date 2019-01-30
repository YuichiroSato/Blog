N <- 20

linkFunction <- function(b1, b2) {
  function(x) {
    1 / (1 + exp(-b1 - b2 * x))
  }
}

distribution <- function(x, linkFunction) {
  l <- match.fun(linkFunction)
  function(y) {
    dbinom(y, N, l(x))
  }
}

xs <- seq(0, 10, by = 2)
ys <- 0:N
xl <- "y"
yl <- "Probability"
legends <- paste0("x = ", xs)
for (b1 in seq(-0.5, 0.5, by = 0.2)) {
  for (b2 in seq(-0.5, 0.5, by = 0.2)) {
    l <- linkFunction(b1, b2)
    title <- paste0("q = 1 / (1 + exp(-(", b1, " + ", b2, "x)))")
    plot(0, 0, type = "n", xlim = c(0, N), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
    for (x in xs) {
      d <- distribution(x, l)
      lines(ys, d(ys), type = "l")
      points(ys, d(ys), pch = x)
    }
    legend("topright", legend = legends, pch = xs)
  }
}
