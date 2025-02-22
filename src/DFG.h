/*
 * ======================================================================
 * DFG.cpp
 * ======================================================================
 * DFG implementation header file.
 *
 * Author : Cheng Tan
 *   Date : July 16, 2019
 */

#ifndef DFG_H
#define DFG_H

#include <string>
#include <llvm/IR/Function.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/Value.h>
#include <llvm/IR/Instruction.h>
#include <llvm/IR/Instructions.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/Support/FileSystem.h>
#include <llvm/IR/Use.h>
#include <llvm/Analysis/CFG.h>
#include <llvm/Analysis/LoopInfo.h>
#include <list>
#include <set>
#include <map>
#include <iostream>

#include "DFGNode.h"
#include "DFGEdge.h"
#include "OperationMap.h"
#include "json.hpp"
#include "Options.h"

using namespace llvm;
using namespace std;
using nlohmann::json;

class DFG {
  private:
    int m_num;
    bool m_CDFGFused;
    bool m_targetFunction;
    bool m_precisionAware;
    list<DFGNode*>* m_orderedNodes;
    list<Loop*>* m_targetLoops;
	Function *m_function;

    //edges of data flow
    list<DFGEdge*> m_DFGEdges;
    list<DFGEdge*> m_ctrlEdges;

	// Just some stuff to allow cloning
	list<string> *m_initPipelinedOpt;
	map<string, int> *m_initExecLatency;

    string changeIns2Str(Value* ins);
    //get value's name or inst's content
    StringRef getValueName(Value* v);
    void DFS_on_DFG(DFGNode*, DFGNode*, list<DFGNode*>*, list<DFGEdge*>*,
        list<DFGEdge*>*, list<list<DFGEdge*>*>*);
    DFGNode* getNode(Value*);
    bool hasNode(Value*);
    DFGEdge* getDFGEdge(DFGNode*, DFGNode*);
    void deleteDFGEdge(DFGNode*, DFGNode*);
    void replaceDFGEdge(DFGNode*, DFGNode*, DFGNode*, DFGNode*);
    bool hasDFGEdge(DFGNode*, DFGNode*);
    DFGEdge* getCtrlEdge(DFGNode*, DFGNode*);
    bool hasCtrlEdge(DFGNode*, DFGNode*);
    bool shouldIgnore(Value*);
    void tuneForBranch();
    void tuneForBitcast();
    void tuneForLoad();
    void tuneForPattern();
    void combineCmpBranch();
    void combineMulAdd();
    void combinePhiAdd();
    void combine(string, string);
    void trimForStandalone();
    void detectMemDataDependency();
    void eliminateOpcode(string);
    bool searchDFS(DFGNode*, DFGNode*, list<DFGNode*>*);
    void connectDFGNodes();
    bool isLiveInInst(BasicBlock*, Instruction*);
    bool containsInst(BasicBlock*, Value*);
    int getInstID(BasicBlock*, Value*);
    // Reorder the DFG nodes (initial CPU execution ordering) in
    // ASAP (as soon as possible) or ALAP (as last as possible)
    // for mapping.
    // void reorderInASAP();
    // void reorderInALAP();
    void reorderInLongest();
    void reorderDFS(set<DFGNode*>*, list<DFGNode*>*,
                    list<DFGNode*>*, DFGNode*);
    void initExecLatency(map<string, int>*);
    void initPipelinedOpt(list<string>*);
    bool isMinimumAndHasNotBeenVisited(set<DFGNode*>*, map<DFGNode*, int>*, DFGNode*);

  public:
	DFG(DFG &old);
    DFG(std::string filename); // Load form a json file.
	DFG(list<DFGNode *> nodes, list<DFGEdge *> edges);
    DFG(Function*, list<Loop*>*, bool, bool, bool, map<string, int>*, list<string>*);
    list<list<DFGNode*>*>* m_cycleNodeLists;
    //initial ordering of insts
    list<DFGNode*> nodes;

    list<DFGNode*>* getBFSOrderedNodes();
    list<DFGNode*>* getDFSOrderedNodes();
    int getNodeCount();
    void construct(Function*);
    void setupCycles();
	void breakCycles(Options*);
	void rejoinCycles();
    list<list<DFGEdge*>*>* calculateCycles();
    list<list<DFGNode*>*>* getCycleLists();
    int getID(DFGNode*);
    bool isLoad(DFGNode*);
    bool isStore(DFGNode*);
    void showOpcodeDistribution();
	std::string asString();
	void removeNode(DFGNode *node);
	void removeNodes(list<DFGNode *>*);
    void generateDot(Function*, bool);
    void generateJSON();
	void addNode(DFGNode *node);
	void addEdge(DFGEdge *edge);
	int getMaxNodeNumber();
	OperationNumber getOperation();
    std::string getSourceFileName();

    void dumpFeatures(std::string filename);
    json computeDistances();

    void dumpFrequencies(std::string filename);
    json computeFrequencies();
};

#endif
