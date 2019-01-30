(define (f a b c)
  (+ a (* 2 b) (* 3 c)))

(define (rec-f n)
  (cond ((< n 3) n)
        (:else (f (rec-f (- n 1))
                  (rec-f (- n 2))
                  (rec-f (- n 3))))))

(define (tail-rec-f n)
  (define (iter i a b c)
    (cond ((<= n i) a)
          (:else (iter (+ i 1) (f a  b  c) a b))))
  (if (< n 3) n (iter 3 4 2 1)))
