ys <- seq(0, 20, by = 5)
qs <- seq(0, 5)

likelihood <- function(y, q) {
  dpois(y, lambda = q)
}

prior <- function(q) {
  dbinom(q, 5, 0.2)
}

marginalOfY <- function(y) {
  joint <- function(q) {
    likelihood(y, q) * prior(q)
  }
  sum(joint(qs))
}

posteriorGivenY <- function(y) {
  function(q) {
    likelihood(y, q) * prior(q) / marginalOfY(y)
  }
}

xl <- "q"
yl <- "Probability"
title <- "Posterior distribution given y"
legends = paste0("y = ", ys)
plot(0, 0, type = "n", xlim = c(0, 5), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
for (y in ys) {
  d <- posteriorGivenY(y)
  lines(qs, d(qs), type = "l")
  points(qs, d(qs), pch = y)
}
legend("topright", legend = legends, pch = ys)
