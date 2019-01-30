linkFunction <- function(b1, b2) {
  function(x, r) {
    1 / (1 + exp(-b1 - b2 * x - r))
  }
}

xl <- "x"
yl <- "q"
xs <- runif(100, 0, 10)
ys <- function(r) {
  linkFunction(-0.5, 0.3)(xs, r)
}
for (sd in seq(0.1, 2.0, by = 0.1)) {
  rs <- rnorm(100, mean = 0, sd = sd)
  ps <- data.frame(x = xs, y = ys(rs))
  title <- paste0("logit(q) = -0.5 + 0.3 * x + r, r ~ N(0, ", sd, ")")
  plot(0, 0, type = "n", xlim = c(0, 10), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
  points(ps, pch = 1)
}
