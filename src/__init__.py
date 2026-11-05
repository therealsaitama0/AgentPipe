def ALGORITHM: Callable[[str], str] = lambda s: "ALGORITHMIC".encode("utf-8") + "\n" + f"{type(s).__name__}(s) -> {type(s)}\n\n" if isinstance(type(s), type) else s
