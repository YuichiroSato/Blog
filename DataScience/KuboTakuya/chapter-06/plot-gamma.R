rs <- seq(0.5, 4.5, by = 1.0)
ys <- seq(0.5, 19.5, by = 1.0)
xl <- "y"
yl <- "Probability"
legends <- paste0("r = ", rs)
for (s in 1:20) {
  title <- paste0("Gamma distribution, p(y|shape = ", s, ", rate = r)")
  plot(0, 0, type = "n", xlim = c(0, 20), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
  for (i in 1:5) {
    lines(ys, dgamma(ys, s, rs[i]), type = "l")
    points(ys, dgamma(ys, s, rs[i]), pch = i)
  }
  legend("topright", legend = legends, pch = 1:5)
}
