(define (cont-frac n d k)
  (define (iter i result)
    (if (< i 1)
        result
        (let ((nk (n i))
              (dk (d i)))
             (iter (- i 1) (/ nk (+ dk result))))))
  (iter k 0))

(define (d x)
  (cond ((= x 2) 2.0)
        ((= (remainder (- x 2) 3) 0) (+ 2 (* 2 (- x 2))))
        (else 1.0)))

(define app-e (+ 2 (cont-frac (lambda (x) 1.0) d 1000000)))
