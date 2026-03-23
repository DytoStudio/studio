import { type Component, createSignal, onCleanup } from 'solid-js';

export interface ResizeDraggerProps {
    direction: 'horizontal' | 'vertical';
    hidden: boolean;
    onResize: (delta: number) => void;
    onDoubleClick: () => void;
    accessibilityValue: number;
}

export const ResizeDragger: Component<ResizeDraggerProps> = (props) => {
    const [resizing, setResizing] = createSignal(false);

    const editBodyWithCursor = (enabled: boolean) => {
        document.body.style.cursor = enabled
            ? props.direction === 'horizontal'
                ? 'col-resize'
                : 'row-resize'
            : '';
        document.body.style.userSelect = enabled ? 'none' : '';
    };

    let lastPosition = 0;
    const handleMouseUp = () => {
        setResizing(false);
        editBodyWithCursor(false);
        window.removeEventListener('mouseup', handleMouseUp);
        window.removeEventListener('mousemove', handleMouseMove);
    };

    const handleMouseMove = (e: MouseEvent) => {
        const positionData =
            props.direction === 'horizontal' ? e.clientX : e.clientY;
        if (resizing()) {
            const delta = positionData - lastPosition;
            props.onResize(delta);
        }
        lastPosition = positionData;
    };

    onCleanup(() => {
        if (resizing()) {
            setResizing(false);
            editBodyWithCursor(false);
            window.removeEventListener('mouseup', handleMouseUp);
            window.removeEventListener('mousemove', handleMouseMove);
        }
    });

    return (
        <>
            {props.hidden ? null : (
                <hr
                    aria-valuenow={props.accessibilityValue}
                    class={`${props.direction === 'horizontal' ? 'w-px h-full relative' : 'w-full h-px relative'} ${resizing() ? 'bg-zinc-600' : 'bg-zinc-600/20'}`}
                    onDblClick={() => {
                        props.onDoubleClick();
                    }}
                    onMouseDown={(e) => {
                        lastPosition =
                            props.direction === 'horizontal'
                                ? e.clientX
                                : e.clientY;
                        setResizing(true);
                        editBodyWithCursor(true);
                        window.addEventListener('mouseup', handleMouseUp);
                        window.addEventListener('mousemove', handleMouseMove);
                    }}
                />
            )}
        </>
    );
};
