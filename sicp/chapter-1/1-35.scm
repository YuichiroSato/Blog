(define tolerance 0.00001)

(define (fixed-point f first-guess)
  (define (close-enough? v1 v2)
    (< (abs (- v1 v2)) tolerance))
  (define (try guess)
    (let ((next (f guess)))
      (if (close-enough? guess next)
          next
          (try next))))
  (try first-guess))

;; let f(x) = 1 + 1/x.
;; Then, f(x) = x when x = (1 + sqrt(5))/2,
;; hence 1 + 2/(1 + sqrt(5)) = (3 + sqrt(5))/(1 + sqrt(5))
;;                           = (3 + 2sqrt(5) - 5)/(1 - 5)
;;                           = (1 + sqrt(5))/2
(define phi (fixed-point (lambda (x) (+ 1 (/ 1 x))) 1.0))
