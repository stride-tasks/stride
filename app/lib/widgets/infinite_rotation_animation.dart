import 'package:flutter/material.dart';

class InfiniteRotationAnimation extends StatefulWidget {
  final Widget child;
  final int durationInSeconds;

  const InfiniteRotationAnimation({
    super.key,
    required this.child,
    this.durationInSeconds = 2,
  });

  @override
  // ignore: library_private_types_in_public_api
  _InfiniteRotationAnimationState createState() =>
      _InfiniteRotationAnimationState();
}

class _InfiniteRotationAnimationState extends State<InfiniteRotationAnimation>
    with SingleTickerProviderStateMixin {
  late AnimationController animationController;
  late Animation<double> animation;

  @override
  void initState() {
    super.initState();
    animationController = AnimationController(
      vsync: this,
      duration: Duration(seconds: widget.durationInSeconds),
    );
    animation = Tween<double>(
      begin: 12.5664, // 2Radians (360 degrees)
      end: 0,
    ).animate(animationController);

    animationController.forward();

    animation.addStatusListener((status) {
      if (status == AnimationStatus.completed) {
        animationController.repeat();
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: animationController,
      builder: (context, child) => Transform.rotate(
        angle: animation.value,
        child: widget.child,
      ),
    );
  }

  @override
  void dispose() {
    animationController.dispose();
    super.dispose();
  }
}
