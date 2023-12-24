import styles from './Underline.module.css';

interface UnderlineProps {
  visible: boolean;
}

const Underline: React.FC<UnderlineProps> = ({ visible }) => {
  return (
    <div
      className={`${styles.container} ${
        visible ? styles.visible : styles.hidden
      }`}
    >
      <svg
        width="238"
        height="12"
        viewBox="0 0 238 12"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M79.5188 4.49363C79.5188 3.63313 79.6888 2.83599 79.978 2.18149C79.9976 2.13694 79.995 2.07632 79.9715 2.03656C79.9481 1.9965 79.9103 1.98846 79.8809 2.0169L77.843 3.98986H25V4.9974H77.843L79.895 6.98399C79.923 7.01105 79.9591 7.00319 79.9814 6.96509C80.0038 6.927 80.0062 6.86899 79.9872 6.82663C79.6925 6.16851 79.5188 5.36326 79.5188 4.49363Z"
          fill="#100F0D"
        />
        <path
          d="M85.6599 1.95791C85.6599 1.95791 85.6364 1.94784 85.59 1.92775C85.5472 1.9104 85.4662 1.87225 85.391 1.8446C85.2584 1.79443 85.0137 1.70974 84.7064 1.66062C84.3992 1.61208 84.0113 1.59698 83.5967 1.71159C83.1853 1.82662 82.7454 2.07067 82.4187 2.53869C82.2552 2.7708 82.1147 3.05301 82.0199 3.38771C81.9255 3.72162 81.8721 4.10253 81.8651 4.51483C81.8575 4.92761 81.8961 5.36966 81.9752 5.8313L82.0053 5.97172L82.0447 6.12755C82.0739 6.23078 82.1064 6.33385 82.1432 6.43628C82.218 6.64042 82.311 6.84156 82.422 7.03665C82.8635 7.82229 83.6146 8.50009 84.5276 9.02062C85.4415 9.54487 86.5102 9.92498 87.654 10.1938C88.7995 10.4616 90.0264 10.6181 91.2991 10.6642C91.4582 10.6679 91.6181 10.6716 91.7785 10.6753C91.9407 10.6753 92.1035 10.675 92.2668 10.6749C92.5835 10.668 92.9089 10.6577 93.2375 10.6348C93.8934 10.5908 94.5592 10.5173 95.2309 10.4065C97.9231 9.9745 100.694 8.99933 103.494 7.77084C104.898 7.16095 106.312 6.48685 107.772 5.84349C108.503 5.52323 109.247 5.21101 110.019 4.94431C110.791 4.68226 111.592 4.44959 112.456 4.40563C112.896 4.38538 113.312 4.4333 113.735 4.5066C114.158 4.58935 114.592 4.70993 115.013 4.97858C115.221 5.1136 115.427 5.29448 115.591 5.5375C115.755 5.77985 115.862 6.08625 115.887 6.38627C115.915 6.68832 115.877 6.97246 115.802 7.23241C115.798 7.25241 115.778 7.30387 115.764 7.34306C115.748 7.38396 115.732 7.4267 115.715 7.46508C115.677 7.53323 115.646 7.60566 115.599 7.6676C115.515 7.7989 115.409 7.90649 115.299 7.9994C115.076 8.18136 114.835 8.27694 114.605 8.33968C114.374 8.40057 114.148 8.42558 113.927 8.43468C113.486 8.45122 113.064 8.40163 112.65 8.33225C111.822 8.18894 111.026 7.95239 110.242 7.68769C108.677 7.15087 107.163 6.48557 105.673 5.78776C105.302 5.61137 104.931 5.43532 104.561 5.25918C104.19 5.07769 103.841 4.92203 103.462 4.77847C102.714 4.48902 101.955 4.22747 101.194 3.98914C99.674 3.51612 98.1417 3.12471 96.6147 2.92421C95.8525 2.82983 95.0882 2.77968 94.3484 2.85314C93.979 2.88928 93.6166 2.96057 93.2796 3.08567C92.947 3.21513 92.6358 3.40012 92.4235 3.67452C92.2081 3.94438 92.0926 4.29538 92.0776 4.67324C92.0508 5.05205 92.1327 5.39918 92.342 5.65847C92.5484 5.91988 92.8627 6.08836 93.1926 6.19554C93.5248 6.30305 93.8779 6.35508 94.2312 6.38089C94.9402 6.42837 95.6542 6.37387 96.3543 6.28451C97.0552 6.1932 97.7451 6.0615 98.4222 5.90931C99.7764 5.60338 101.08 5.21869 102.328 4.80042C102.951 4.5904 103.561 4.37136 104.154 4.13867C104.449 4.02156 104.743 3.90308 105.021 3.77074C105.15 3.7063 105.303 3.62169 105.443 3.55886C105.586 3.492 105.728 3.42926 105.871 3.36787C108.151 2.41215 110.33 1.82782 112.29 1.55094C114.249 1.27842 116.003 1.35713 117.372 1.79049C118.057 2.00129 118.639 2.29923 119.105 2.62772C119.569 2.95929 119.921 3.31109 120.174 3.62724C120.431 3.94041 120.587 4.21626 120.696 4.39717C120.796 4.58524 120.847 4.68251 120.847 4.68251C120.847 4.68251 120.795 4.58629 120.693 4.39981C120.581 4.22084 120.422 3.94785 120.162 3.63959C119.906 3.32869 119.551 2.98446 119.085 2.66257C118.618 2.3436 118.037 2.05727 117.355 1.85928C115.992 1.45101 114.25 1.40069 112.308 1.70249C110.365 2.00831 108.202 2.62564 105.955 3.60862C105.816 3.67161 105.676 3.7358 105.537 3.80332C105.394 3.87042 105.272 3.94147 105.12 4.02075C104.831 4.16383 104.539 4.28666 104.241 4.41013C103.647 4.6542 103.037 4.88421 102.412 5.10552C101.163 5.54613 99.8566 5.95415 98.4926 6.28554C97.8104 6.45007 97.1134 6.59485 96.4001 6.6993C95.6869 6.80171 94.9566 6.87067 94.2085 6.83268C93.8349 6.81115 93.4557 6.76308 93.078 6.64791C92.7049 6.53105 92.3147 6.34539 92.0176 5.98512C91.714 5.6296 91.5942 5.10061 91.6255 4.64743C91.6358 4.18842 91.782 3.69781 92.0697 3.33052C92.3535 2.95743 92.7382 2.7291 93.1203 2.57344C93.5067 2.42311 93.9036 2.34052 94.2994 2.29528C95.0925 2.20423 95.8846 2.24552 96.6714 2.32995C98.2453 2.51102 99.8043 2.88436 101.353 3.33924C102.127 3.56877 102.898 3.82074 103.667 4.10439C104.049 4.24149 104.448 4.41224 104.814 4.5846C105.185 4.75436 105.557 4.92389 105.928 5.09375C107.415 5.76276 108.921 6.39557 110.453 6.89382C111.218 7.1395 111.992 7.35418 112.762 7.4746C113.145 7.53299 113.53 7.56953 113.895 7.54913C114.256 7.53098 114.612 7.44693 114.818 7.27104C114.867 7.22644 114.912 7.18199 114.943 7.12837C114.963 7.10497 114.971 7.07239 114.989 7.04634L115.001 7.0144C115.005 7.00174 115.01 7.00319 115.017 6.96835C115.065 6.80188 115.081 6.63073 115.065 6.48621C115.049 6.33982 115.003 6.2149 114.925 6.09972C114.845 5.98593 114.731 5.88189 114.593 5.79559C114.315 5.62114 113.961 5.5171 113.598 5.45386C113.233 5.39588 112.851 5.3587 112.496 5.38265C111.768 5.42951 111.022 5.65106 110.293 5.91309C109.562 6.17861 108.839 6.49444 108.123 6.82133C106.691 7.47873 105.28 8.17845 103.868 8.81826C101.05 10.1071 98.21 11.1625 95.4023 11.6593C94.7018 11.7865 94.0041 11.8751 93.3142 11.9325C92.9699 11.962 92.626 11.9787 92.2788 11.9915C92.1101 11.9942 91.9422 11.9971 91.7749 12C91.6061 11.9987 91.4379 11.9975 91.2705 11.9963C89.9312 11.969 88.638 11.8264 87.4073 11.5594C86.178 11.2898 85.0063 10.9005 83.9446 10.3144C83.4148 10.0212 82.9132 9.67697 82.4608 9.2688C82.009 8.86125 81.6088 8.38428 81.2956 7.85101C81.1393 7.58485 81.005 7.30506 80.8958 7.01851C80.8416 6.87455 80.7938 6.72961 80.7507 6.58411L80.6921 6.36499C80.6746 6.28612 80.6572 6.20747 80.6398 6.12941C80.539 5.58315 80.4815 5.03512 80.4843 4.49563C80.4867 3.95656 80.5497 3.42424 80.6846 2.9238C80.8189 2.42327 81.0338 1.95897 81.3028 1.56731C81.5713 1.17413 81.896 0.858461 82.2326 0.629321C82.5689 0.398261 82.9134 0.247352 83.2378 0.15155C83.563 0.0554795 83.8696 0.01274 84.1473 0.00369667C84.4254 -0.0080721 84.6759 0.00927219 84.8966 0.0414608C85.3404 0.106292 85.6553 0.208474 85.8933 0.296079C86.083 0.3727 86.181 0.412053 86.181 0.412053L85.6599 1.95791Z"
          fill="#100F0D"
        />
        <path
          d="M87.1438 2.07803C87.1438 3.08271 86.4149 3.89746 85.5156 3.89746C84.6164 3.89746 83.8873 3.08271 83.8873 2.07803C83.8873 1.07335 84.6164 0.258432 85.5156 0.258432C86.4149 0.258432 87.1438 1.07335 87.1438 2.07803Z"
          fill="#100F0D"
        />
        <path
          d="M158.481 4.49363C158.481 3.63313 158.311 2.83599 158.022 2.18149C158.002 2.13694 158.005 2.07632 158.028 2.03656C158.052 1.9965 158.09 1.98846 158.119 2.0169L160.157 3.98986H213V4.9974H160.157L158.105 6.98399C158.077 7.01105 158.041 7.00319 158.019 6.96509C157.996 6.927 157.994 6.86899 158.013 6.82663C158.308 6.16851 158.481 5.36326 158.481 4.49363Z"
          fill="#100F0D"
        />
        <path
          d="M151.818 0.412053C151.818 0.412053 151.916 0.3727 152.106 0.296079C152.344 0.208474 152.659 0.106292 153.103 0.0414608C153.323 0.00927219 153.574 -0.0080721 153.852 0.00369667C154.13 0.01274 154.436 0.0553143 154.761 0.15155C155.086 0.247352 155.43 0.398013 155.767 0.629321C156.103 0.858461 156.428 1.17413 156.696 1.56707C156.965 1.95872 157.18 2.42311 157.315 2.92355C157.45 3.4241 157.513 3.95631 157.515 4.49563C157.518 5.03512 157.461 5.58315 157.36 6.12924C157.342 6.20747 157.325 6.28595 157.307 6.36482L157.249 6.58411C157.206 6.72961 157.158 6.87455 157.104 7.01828C156.994 7.30492 156.86 7.5846 156.704 7.85101C156.391 8.38428 155.99 8.86125 155.538 9.26863C155.086 9.67697 154.585 10.0212 154.055 10.3144C152.993 10.9005 151.821 11.2898 150.592 11.5594C149.361 11.8264 148.068 11.969 146.729 11.9963C146.561 11.9975 146.393 11.9987 146.224 12C146.057 11.9971 145.889 11.9942 145.721 11.9915C145.373 11.9787 145.03 11.962 144.685 11.9325C143.995 11.8751 143.298 11.7865 142.597 11.6593C139.789 11.1625 136.95 10.1071 134.131 8.81826C132.719 8.17862 131.308 7.47873 129.876 6.82133C129.16 6.49444 128.438 6.17861 127.707 5.91309C126.977 5.65106 126.231 5.42951 125.503 5.38265C125.149 5.3587 124.767 5.39588 124.401 5.45386C124.039 5.5171 123.684 5.62114 123.406 5.79559C123.268 5.88189 123.154 5.98593 123.075 6.09972C122.996 6.2149 122.95 6.33982 122.934 6.48604C122.918 6.63073 122.934 6.80188 122.982 6.96835C122.99 7.00319 122.994 7.00174 122.999 7.0144L123.011 7.04634C123.028 7.07239 123.036 7.10497 123.056 7.12837C123.087 7.18199 123.133 7.22644 123.182 7.27104C123.387 7.44693 123.744 7.53098 124.104 7.54913C124.469 7.56953 124.854 7.53299 125.238 7.4746C126.008 7.35418 126.781 7.1395 127.546 6.89382C129.078 6.39557 130.584 5.76276 132.071 5.09375C132.443 4.92389 132.814 4.75436 133.185 4.5846C133.552 4.41224 133.95 4.24149 134.332 4.10439C135.101 3.82074 135.872 3.56877 136.646 3.33924C138.195 2.88436 139.754 2.51102 141.328 2.32995C142.115 2.24552 142.907 2.20423 143.7 2.29528C144.096 2.34052 144.493 2.42311 144.879 2.57344C145.261 2.7291 145.646 2.95743 145.93 3.33052C146.217 3.69781 146.364 4.18842 146.374 4.64743C146.405 5.10061 146.285 5.6296 145.982 5.98512C145.685 6.34539 145.295 6.53105 144.921 6.64791C144.544 6.76308 144.165 6.81115 143.791 6.83268C143.043 6.87067 142.312 6.80171 141.599 6.6993C140.886 6.59485 140.189 6.45007 139.507 6.28554C138.143 5.95415 136.836 5.54613 135.587 5.10552C134.963 4.88421 134.352 4.6542 133.758 4.41013C133.461 4.2865 133.168 4.16367 132.879 4.02075C132.727 3.94147 132.605 3.87042 132.463 3.80332C132.324 3.7358 132.184 3.67161 132.044 3.60862C129.798 2.62564 127.634 2.00831 125.691 1.70249C123.749 1.40069 122.007 1.45101 120.644 1.85928C119.962 2.05727 119.381 2.3436 118.914 2.66257C118.448 2.98446 118.093 3.32869 117.838 3.63959C117.578 3.94785 117.418 4.22084 117.307 4.39981C117.205 4.58629 117.152 4.68251 117.152 4.68251C117.152 4.68251 117.203 4.58524 117.303 4.39717C117.412 4.21626 117.569 3.94041 117.825 3.62724C118.078 3.31109 118.43 2.95929 118.895 2.62772C119.361 2.29923 119.943 2.00129 120.627 1.79049C121.996 1.35713 123.751 1.27842 125.709 1.55094C127.669 1.82782 129.849 2.41215 132.129 3.36771C132.271 3.42901 132.413 3.49176 132.556 3.55869C132.697 3.62144 132.849 3.7063 132.978 3.77074C133.256 3.90308 133.55 4.02156 133.846 4.13867C134.438 4.37111 135.048 4.5904 135.672 4.80042C136.919 5.21869 138.223 5.60316 139.577 5.90931C140.254 6.0615 140.944 6.1932 141.645 6.28451C142.345 6.37387 143.059 6.42837 143.768 6.38089C144.122 6.35508 144.474 6.30305 144.807 6.19554C145.137 6.08836 145.451 5.91988 145.657 5.65847C145.867 5.39918 145.949 5.05205 145.922 4.67324C145.907 4.29538 145.791 3.94438 145.576 3.67452C145.363 3.40012 145.052 3.21513 144.72 3.08567C144.383 2.96057 144.02 2.88928 143.651 2.85314C142.911 2.77968 142.147 2.82983 141.385 2.92421C139.858 3.12471 138.325 3.51612 136.805 3.98914C136.045 4.22747 135.286 4.48902 134.537 4.77847C134.158 4.92203 133.81 5.07769 133.438 5.25918C133.068 5.43532 132.698 5.61137 132.327 5.78776C130.837 6.48557 129.322 7.15087 127.757 7.68769C126.973 7.95239 126.178 8.18894 125.35 8.33225C124.935 8.40163 124.513 8.45122 124.072 8.43468C123.851 8.42558 123.626 8.40057 123.394 8.33968C123.164 8.27694 122.923 8.18136 122.7 7.9994C122.59 7.90649 122.484 7.7989 122.4 7.6676C122.354 7.60566 122.322 7.53323 122.285 7.46508C122.267 7.4267 122.251 7.38396 122.236 7.34306C122.221 7.30387 122.202 7.25241 122.198 7.23241C122.123 6.97246 122.085 6.68832 122.112 6.38627C122.138 6.08609 122.244 5.77985 122.408 5.5375C122.572 5.29448 122.778 5.1136 122.987 4.97858C123.408 4.70993 123.842 4.58935 124.264 4.5066C124.687 4.4333 125.103 4.38538 125.544 4.40563C126.407 4.44959 127.208 4.68226 127.981 4.94431C128.752 5.21101 129.496 5.52323 130.227 5.84349C131.688 6.48685 133.101 7.16095 134.505 7.77084C137.305 8.99933 140.076 9.9745 142.768 10.4065C143.44 10.5173 144.106 10.5908 144.762 10.6348C145.091 10.6577 145.416 10.668 145.732 10.6749C145.896 10.675 146.059 10.6753 146.221 10.6753C146.381 10.6716 146.541 10.6679 146.7 10.6642C147.973 10.6181 149.2 10.4616 150.345 10.1938C151.489 9.92498 152.558 9.54487 153.472 9.02062C154.385 8.50009 155.136 7.82229 155.577 7.03665C155.688 6.84131 155.781 6.64042 155.856 6.43628C155.893 6.33385 155.925 6.23078 155.955 6.12738L155.994 5.97172L156.024 5.8313C156.103 5.36941 156.142 4.92736 156.134 4.51483C156.127 4.10253 156.074 3.72162 155.979 3.38755C155.885 3.05301 155.744 2.7708 155.581 2.53853C155.254 2.07067 154.814 1.82645 154.403 1.71159C153.988 1.59698 153.6 1.61208 153.293 1.66062C152.986 1.70974 152.741 1.79443 152.608 1.8446C152.533 1.87225 152.452 1.9104 152.409 1.92775C152.363 1.94784 152.339 1.95791 152.339 1.95791L151.818 0.412053Z"
          fill="#100F0D"
        />
        <path
          d="M150.855 2.07803C150.855 3.08271 151.584 3.89746 152.484 3.89746C153.383 3.89746 154.112 3.08271 154.112 2.07803C154.112 1.07335 153.383 0.258432 152.484 0.258432C151.584 0.258432 150.855 1.07335 150.855 2.07803Z"
          fill="#100F0D"
        />
      </svg>
    </div>
  );
};

export default Underline;
