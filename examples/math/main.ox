extern {
    func ExitProcess(exitCode: u32)
}

func exit(code: u32) {
    ExitProcess(code)
}

func main() {
    var a: u32 = 2 + 2

    exit(a)
}
