import './style/stars.scss'

export default () => {
    return (
        <div class='shop-stars-fnd'>
            {[50, 75, 100, 150, 250, 350, 500, 750, 1000, 1500, 2500].map(s => (
                <div>{s}</div>
            ))}
        </div>
    )
}
