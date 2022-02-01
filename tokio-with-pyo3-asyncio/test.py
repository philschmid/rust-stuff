from transformers import pipeline
import asyncio

clx = pipeline("text-classification")


def run():
    return clx("I like you i love you.")


async def async_run():
    return clx("I like you i love you.")


import asyncio
import time


async def say_after(delay, what):
    await asyncio.sleep(delay)
    print(what)


async def main():
    await async_run()
    print("after")
    await async_run()
    print("after2")

    print(f"finished at {time.strftime('%X')}")


asyncio.run(main())
