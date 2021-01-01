import {AnimationSequence, TemplateElement} from "../websocket/types";

declare global {
    interface Window {
        zagreusRuntimeState: ZagreusRuntimeState,
    }
}

export interface AlignmentState {
    elementBoundingBox: DOMRect,
}

export interface ZagreusRuntimeState {
    animationsSequences: AnimationSequence[],
    elements: TemplateElement[],
    alignmentStates: { [key in string]: AlignmentState },
}

if (!window.zagreusRuntimeState) {
    window.zagreusRuntimeState = {
        animationsSequences: [],
        elements: [],
        alignmentStates: undefined,
    }
}

export const getZagreusState = (): ZagreusRuntimeState => {
    return window.zagreusRuntimeState;
}


