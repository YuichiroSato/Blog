(define (improve guess x)
  (/ (+ (* 2 guess) (/ x (square guess))) 3))

(define (cubic-root-iter guess x)
  (let ((guess1 (improve guess x)))
       (if (good-enough? guess guess1)
           guess
           (cubic-root-iter guess1 x))))

(define (good-enough? guess0 guess1)
  (< (abs (- guess0 guess1)) (* guess0 0.001)))

(define (cubic-root x)
  (cubic-root-iter 1.0 x))
