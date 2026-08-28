extern {
    func ExitProcess(exitCode: u32)
}

func main() {
    ExitProcess(42)
}
