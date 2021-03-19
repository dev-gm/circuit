import sys, pygame
from math import sqrt
from typing import Tuple, List
from pygame import Surface, Color, Rect


class Node:
    def __init__(self, pos: Tuple[int, int], end_nodes: list = []):
        self.pos = pos
        self.end_nodes = end_nodes

    def connect_to(self, node):
        if node is not self:
            self.end_nodes.append(node)

    def near(self, pos: Tuple[int, int]) -> float:
        return sqrt(sum(abs(pos[i]-self.pos[i])**2 for i in range(2)))

    def render(self, screen: Surface):
        pygame.draw.rect(screen, Color(0, 0, 0), Rect([pos for pos in self.pos], (50, 50)))
        for node in self.end_nodes:
            pygame.draw.line(screen, Color(0, 0, 0), self.pos, node.pos)

    @staticmethod
    def get_endpoints(nodes: list, points: List[Tuple[int, int]]) -> tuple:
        output = []
        for point in points:
            nearest_nodes = [node.near(point) for node in nodes]
            output.append(nodes[nearest_nodes.index(min(nearest_nodes))] if nodes else None)
        return tuple(output)


def main():
    pygame.display.init()
    screen = pygame.display.set_mode((1920, 1080))
    pygame.display.set_caption("Make Circuits")
    nodes = []
    current = [(), ()]
    background = Color(255, 255, 255)
    shift = False
    while True:
        pygame.display.update()
        screen.fill(background)
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                sys.exit()
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_LSHIFT:
                    shift = True
            elif event.type == pygame.KEYUP:
                if event.key == pygame.K_LSHIFT:
                    shift = False
            elif event.type == pygame.MOUSEBUTTONDOWN:
                if shift:
                    nodes.append(Node(pygame.mouse.get_pos()))
                else:
                    current[0] = pygame.mouse.get_pos()
            elif event.type == pygame.MOUSEBUTTONUP:
                if current[0]:
                    current[1] = pygame.mouse.get_pos()
                    endpoints = Node.get_endpoints(nodes, current)
                    if endpoints[0] and endpoints[1]:
                        endpoints[0].connect_to(endpoints[1])
                    current = [(), ()]
        for node in nodes:
            node.render(screen)

if __name__ == "__main__":
    main()
