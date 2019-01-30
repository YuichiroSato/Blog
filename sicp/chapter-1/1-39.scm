(define (cont-frac n d k)
  (define (iter i result)
    (if (< i 1)
        result
        (let ((nk (n i))
              (dk (d i)))
             (iter (- i 1) (/ nk (- dk result))))))
  (iter k 0))

(define (n x) (lambda (i) (if (= i 1) x (* x x))))

(define (d i) (- (* i 2) 1))

(define (tan-cf x k) (cont-frac (n x) d k))
