from transformers import pipeline

clx = pipeline("text-classification")


async def handle(input: str):
    return clx(input)


# class Handler(object):
#     def __init__(self, task):
#         self.pipeline = pipeline(task)

#     async def __call__(self, inputs):
#         return self.pipeline(inputs)
