import React from 'react';

interface CubeProps {
    size: number;
    x: number;
    y: number;
    topColor: string;
    bottomColor: string;
    leftColor: string;
    rightColor: string;
    frontColor: string;
    backColor: string;
}

interface BlockProps {
    color: string;
    x: number;
    y: number;
}

const Block: React.FC<BlockProps> = ({ color, x, y }) => (
    <div
        style={{
            position: 'absolute',
            width: '33.33%',
            height: '33.33%',
            backgroundColor: color,
            left: `${x * 33.33}%`,
            top: `${y * 33.33}%`,
            border: '1px solid black',
        }}
    />
);

const Face: React.FC<{ color: string; rotate: string; translate: string }> = ({ color, rotate, translate }) => (
    <div
        style={{
            position: 'absolute',
            width: '100%',
            height: '100%',
            transform: `${rotate} ${translate}`,
            backfaceVisibility: 'hidden',
        }}
    >
        {Array.from({ length: 9 }).map((_, index) => (
            <Block key={index} color={color} x={index % 3} y={Math.floor(index / 3)} />
        ))}
    </div>
);

const Cube: React.FC<CubeProps> = ({
    size,
    x,
    y,
    topColor,
    bottomColor,
    leftColor,
    rightColor,
    frontColor,
    backColor,
}) => {
    const faceSize = size - 2; // Reduce face size to account for spacing
    const translateZ = faceSize / 2 + 1; // Adjust translateZ for correct face positioning

    const cubeStyle: React.CSSProperties = {
        position: 'absolute',
        transformStyle: 'preserve-3d',
        transform: `translate3d(${x + 50}px, ${y+ 50}px, 0px) rotateX(-45deg) rotateY(45deg)`,
        width: `${size}px`,
        height: `${size}px`,
    };

    return (
        <div style={cubeStyle}>
            <Face color={topColor} rotate="rotateX(90deg)" translate={`translateZ(${translateZ}px)`} />
            <Face color={leftColor} rotate="rotateY(-90deg)" translate={`translateZ(${translateZ}px)`} />
            <Face color={frontColor} rotate="rotateY(0deg)" translate={`translateZ(${translateZ}px)`} />
            <Face color={rightColor} rotate="rotateY(90deg)" translate={`translateZ(${translateZ}px)`} />
            <Face color={backColor} rotate="rotateY(180deg)" translate={`translateZ(${translateZ}px)`} />
            <Face color={bottomColor} rotate="rotateX(-90deg)" translate={`translateZ(${translateZ}px)`} />
        </div>
    );
};

const RubiksCube2D: React.FC = () => {
    const cubeSize = 90;
    const spacing = cubeSize * 4;

    const cubes: CubeProps[] = [
        {
            x: 0,
            y: spacing * 0.1,
            size: cubeSize * 0.9,
            topColor: 'yellow',
            bottomColor: 'white',
            leftColor: 'orange',
            rightColor: 'red',
            frontColor: 'green',
            backColor: 'green',
        },
        {
            x: spacing * 1.1,
            y: 0,
            size: cubeSize * 0.8,
            topColor: 'yellow',
            bottomColor: 'white',
            leftColor: 'red',
            rightColor: 'red',
            frontColor: 'blue',
            backColor: 'green',
        },
        {
            x: spacing / 2,
            y: spacing / 2,
            size: cubeSize * 1.1,
            topColor: 'yellow',
            bottomColor: 'white',
            leftColor: 'orange',
            rightColor: 'red',
            frontColor: 'blue',
            backColor: 'green',
        },
        {
            x: spacing * 0.55,
            y: spacing * 0.95,
            size: cubeSize * 0.85,
            topColor: 'gainsboro',
            bottomColor: 'yellow',
            leftColor: 'orange',
            rightColor: 'red',
            frontColor: 'blue',
            backColor: 'green',
        },
    ];

    return (
        <div
            style={{
                position: 'relative',
                width: '600px', // Increased width to accommodate spacing. Adjust as needed.
                height: '600px', // Increased height to accommodate spacing. Adjust as needed.
                margin: '5px auto',
                perspective: '1000px',
                perspectiveOrigin: '250px 250px', // Adjusted perspective origin
                backgroundColor: 'black', // Changed background color to black
            }}
        >
            {cubes.map((cube, index) => (
                <Cube
                    key={index}
                    size={cube.size}
                    x={cube.x}
                    y={cube.y}
                    topColor={cube.topColor}
                    bottomColor={cube.bottomColor}
                    leftColor={cube.leftColor}
                    rightColor={cube.rightColor}
                    frontColor={cube.frontColor}
                    backColor={cube.backColor}
                />
            ))}
        </div>
    );
};

export default RubiksCube2D;