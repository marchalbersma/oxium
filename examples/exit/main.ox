extern {
    func ExitProcess(exitCode: u32)
}

func exit(code: u32) {
    ExitProcess(code)
}

func main() {
    exit(69)
}
