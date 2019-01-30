(define (make-point x y)
  (cons x y))

(define (x-point p)
  (car p))

(define (y-point p)
  (cdr p))

;;
;; p1              p2
;;   --------------
;;   |            |
;;   |            |
;;   |            |
;;   --------------
;; p4              p3 
;;
(define (make-rectangle p1 p2 p3 p4)
  (cons p1 (cons p2 (cons p3 p4))))

;;       width
;;    <---------->
;; (x,y)
;;   -------------- ^
;;   |            | | hight
;;   |            | |
;;   |            | |
;;   -------------- \/
;;
(define (make-rectangle2 x y width hight)
  (make-rectangle
    (make-point x y)
    (make-point (+ x width) y)
    (make-point (+ x width) (+ y hight))
    (make-point x (+ y hight))))

(define (square a) (* a a))

(define (distance p1 p2)
  (let ((x1 (x-point p1))
        (y1 (y-point p1))
        (x2 (x-point p2))
        (y2 (y-point p2)))
    (sqrt (+ (square (- x1 x2))
             (square (- y1 y2))))))


(define (width-rectangle rectangle)
  (distance (car rectangle)
            (car (cdr rectangle))))

(define (height-rectangle rectangle)
  (distance (car (cdr rectangle))
            (car (cdr (cdr rectangle)))))

(define (area rectangle)
  (* (width-rectangle rectangle)
     (height-rectangle rectangle)))

(define (perimeter rectangle)
  (* 2 (+ (width-rectangle rectangle)
          (height-rectangle rectangle))))
