
if __name__ ==  "__main__":
    base = [2, 3, 5, 7];

    vgen = Vdcorput(2);
    for _ in range(10):
        print("{}".format(vgen.pop()))

    cgen = Circle(2);
    for _ in range(10):
        print("{:?}".format(cgen.pop()))

    hgen = Halton(base);
    for _ in range(10):
        print("{:?}".format(hgen.pop()))

    sgen = Sphere(base);
    for _ in range(10):
        print("{:?}".format(sgen.pop()))

    s3fgen = Sphere3Hopf(base);
    for _ in range(10):
        print("{:?}".format(s3fgen.pop()))
