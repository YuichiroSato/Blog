(define (main args)
  (let ((p1 (make-interval 5 7))
        (p2 (make-interval -2 3))
        (p3 (make-interval -8 -6)))
    (print (mul-interval p1 p1))
    (print (mul-interval p1 p2))
    (print (mul-interval p1 p3))
    (print (mul-interval p2 p1))
    (print (mul-interval p2 p2))
    (print (mul-interval p2 p3))
    (print (mul-interval p3 p1))
    (print (mul-interval p3 p2))
    (print (mul-interval p3 p3))))

(define (mul-interval x y)
  (let ((l1 (lower-bound x))
        (u1 (upper-bound x))
        (l2 (lower-bound y))
        (u2 (upper-bound y)))
    (cond ((positives? (list l1 u1 l2 u2)) (make-interval (* l1 l2) (* u1 u2)))
          ((and (positives? (list l1 u1 u2)) (negative? l2)) (make-interval (* u1 l2) (* u1 u2)))
          ((and (positives? (list l1 u1)) (negatives? (list l2 u2))) (make-interval (* u1 l2) (* l1 u2)))
          ((and (negative? l1) (positives? (list u1 l2 u2))) (make-interval (* l1 u2) (* u1 u2)))
          ((and (negatives? (list l1 l2 u2)) (positive? u1)) (make-interval (* u1 l2) (* l1 u2)))
          ((and (negatives? (list l1 u1)) (positives? (list l2 u2))) (make-interval (* l1 u2) (* u1 l2)))
          ((and (negatives? (list l1 u1 l2)) (positive? u2)) (make-interval (* l1 u2) (* l1 l2)))
          ((and (negatives? (list l1 u1 l2 u2))) (make-interval (* l1 u1) (* l2 u2)))
          (:else (mul-interval2 x y)))))

(define (mul-interval2 x y)
  (let ((p1 (* (lower-bound x) (lower-bound y)))
        (p2 (* (lower-bound x) (upper-bound y)))
        (p3 (* (upper-bound x) (lower-bound y)))
        (p4 (* (upper-bound x) (upper-bound y))))
    (make-interval (min p1 p2 p3 p4)
                   (max p1 p2 p3 p4))))

(define (positives? ls)
  (if (null? ls)
      #t
      (and (positive? (car ls)) (positives? (cdr ls)))))

(define (negatives? ls)
  (if (null? ls)
      #t
      (and (negative? (car ls)) (negatives? (cdr ls)))))

(define (make-interval a b) (cons a b))

(define (lower-bound x) (car x))

(define (upper-bound x) (cdr x))
