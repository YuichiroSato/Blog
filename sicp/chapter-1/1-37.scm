(define (cont-frac n d k)
  (define (iter i result)
    (if (< i 1)
        result
        (let ((nk (n i))
              (dk (d i)))
             (iter (- i 1) (/ nk (+ dk result))))))
  (iter k 0))

(define (cont-frac2 n d k)
  (define (iter i)
    (if (> i k)
        (/ (n k) (d k))
        (/ (n i) (+ (d i) (iter (+ i 1))))))
  (iter 1))

(define phi (cont-frac (lambda (x) 1.0) (lambda (x) 1.0) 100))
