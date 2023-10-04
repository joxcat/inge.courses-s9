__author__ = 'Jacques saraydaryan'

from global_planner_short_path_student.ShortPathMethods.AbstractShortPath import AbstractShortPath
from visualization_msgs.msg import MarkerArray
import math
import rclpy
import time

# import sys
# sys.path.append('../')


class Dijsktra(AbstractShortPath):
    SLEEP_TIME_BEFORE_NEXT_ITERATION = 0.01

    def __init__(self):
        print('')


    def goto(self, source, target, matrix, pub_marker, marker_container):
        prev = {}
        ### TODO
        ###########################################################
        ################### Function Paramters ###################
        ###########################################################
        ### source: coordinate of the robot position source['x'] return the x position, source['y'] return the y position
        ###
        ### target: coordinate of the target position target['x'] return the x position, target['y'] return the y position
        ###
        ### matrix: rescaled map (including obstacles) matrix[i][j] return the value of the cell i,j of the matrix
        ###
        ### elf.MAP_OBSTACLE_VALUE: value of an obstacle into the matrix (-100)
        ###
        ### pub_marker: marker publisher to visualize information into rviz (usage pub_marker.publish(marker_container) )
        ###
        ### marker_container: marker container where where new marker are added as point
        ###
        ###########################################################
        ################### Function Toolboxes ###################
        ###########################################################
        #   # create a visual information
        #   self.createFontierUnitMarker(v, marker_container)
        #
        #    # publish visual information
        #    pub_marker.publish(marker_container)
        #
        #    # create a visual information
        #    self.createClosedMarker(u, marker_container)
        ###
        ### prev:  disctionary holding node precedence
        ### CAUTION prev dictionary has to be completed as follow:
        ###
        ### prev[str(v['x']) + '_' + str(v['y'])] = str(u['x']) + '_' + str(u['y'])
        ###
        ### where v['x'] return the x position of the node v in the resized map
        ### where v['y'] return the y position of the node v in the resized map
        # Dictionary that holds node score
        fscore = {}
        prev = {}
        # List that holds the nodes to process
        unvisited = []
        INF = 9999

        # Condition to stop the path finding algo
        isEnd = False
        print('start processing')

        # Intialisation
        for x in range(len(matrix)):
            for y in range(len(matrix[x])):
                if matrix[x][y] != self.MAP_OBSTACLE_VALUE:
                    # all nodes receive a score of INF
                    fscore[str(x) + '_' + str(y)] = INF
                    prev[str(x) + '_' + str(y)] = None
                    # all nodes are added to the list to process
                    unvisited.append({'x': x, 'y': y})
        # score of the start node is set to 0
        fscore[str(source['x']) + '_' + str(source['y'])] = 0
        print('Source: ' + str(source['x']) + '_' + str(source['y']))
        print('end initialisation phase')

        # while their is node to process or goal is reached (early exit)
        while len(unvisited) != 0 and not isEnd:
            # get the node with the lowest score
            u = self.minScore(fscore, unvisited)
            print('current Node:' + str(u))
            # remove the current node to the node to process list
            unvisited.remove(u)
            # create a visual information
            #self.createClosedMarker(u, marker_container)
            self.createClosedMarkerPt(u, marker_container)

            # get the list of the neighbors of the current node
            currentNeighbors = self.getNeighbors(u, matrix)
            # for all neighbors
            for v in currentNeighbors:
                # check that the current node has not already be processed
                if self.inU(v, unvisited):
                    # create a visual information
                    # self.createFontierUnitMarker(v, marker_array)
                    self.createFontierUnitMarkerPt(v, marker_container)
                    # update the score of the current neighbor with the estimate distance between the neighbors and
                    # the target (heuristic)
                    v_idx = str(v['x']) + '_' + str(v['y'])
                    current_score = fscore[str(u['x']) + '_' + str(u['y'])] + self.hn(u, v)
                    if current_score < fscore[v_idx]:
                        fscore[v_idx] = current_score
                        prev[v_idx] = u
                    # check if the current neighbor is the target
                    if str(v_idx) == str(target['x']) + '_' + str(target['y']):
                        # end the path computation
                        isEnd = True
            # publish visual information
            pub_marker.publish(marker_container)
            #marker_container = self._create_marker_container()
            # wait before next iteration
            time.sleep(self.SLEEP_TIME_BEFORE_NEXT_ITERATION)
            #rospy.sleep(self.SLEEP_TIME_BEFORE_NEXT_ITERATION)
        
        print(str(prev))
        return prev
    
    def minScore(self, fscore, unvisited):
        """ Return the node that has the lowest score, information return like u={'x':5,'y':3}"""
        min = 9999
        min_coord = ''
        for n in unvisited:
            if fscore[str(n['x']) + '_' + str(n['y'])] < min:
                min = fscore[str(n['x']) + '_' + str(n['y'])]
                min_coord = n
        return min_coord

    def getNeighbors(self, currentNode, matrix):
        """ Compute Neighbors of the current point, Return the list of the point neighbors in Cfree"""
        x_c = currentNode['x']
        y_c = currentNode['y']
        neighbors = []
        self.checkAndAdd(neighbors, x_c + 1, y_c, matrix)
        self.checkAndAdd(neighbors, x_c, y_c + 1, matrix)
        self.checkAndAdd(neighbors, x_c - 1, y_c, matrix)
        self.checkAndAdd(neighbors, x_c, y_c - 1, matrix)
        return neighbors

    def checkAndAdd(self, neighbors, x, y, matrix):
        """ Check that the candidate neighbor is valid == not an obstacle, in current bound, add the nieghbor node to
        the node list """
        if x > 0 and x < self.map_width and y > 0 and y < self.map_height:
            if matrix[y][x] != self.MAP_OBSTACLE_VALUE:
                neighbors.append({'x': x, 'y': y})
        return neighbors

    def hn(self, source, destination):
        """Compute the distance between the given node and the target, the result is an estimation of the distance
        without taking into account obstacles """
        return math.sqrt(math.pow(source['x'] - destination['x'], 2) + math.pow(source['y'] - destination['y'], 2))

    def inU(self, v, unvisited):
        """ Check if the node is into the list, return boolean """
        return v in unvisited
